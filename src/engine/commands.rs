use std::{thread, process};
use std::io::Write;
use std::sync::Arc;
use network::server;
use data::{account, state, database, log, profile, tenv};
use util::{helper, genesis, demo};
use std::sync::RwLock;
use engine::consensus;
use std::collections::HashMap;
use std::time::Duration;

//====================================================================
// COMMAND FUNCTIONS
// Contains functions called by the CLI.
//====================================================================

// Drops all database tables
pub fn drop_all(){
        let conn = database::connect_db();
        account::drop_account_table(&conn);
        state::drop_state_table(&conn);
        log::drop_log_table(&conn);
        profile::drop_profile_table(&conn);
        database::close_db(conn);
}

// Drops database and loads Genesis state.
// Input    Boolean     Activation flag
pub fn load_genesis(init: bool){
    // If init is true, it means its a fresh install so there isn't a need for a prompt
    if !init {
        println!("=>> Loading Genesis state erases the database. Continue? (y/n)");
        let yn = helper::read_yn();
        if !yn {
            return;
        }
        drop_all();
        process::exit(1);
    }
    let genesis_state = genesis::get_genesis();
    let conn = database::connect_db();
    state::save_state(&genesis_state, &conn);
    database::close_db(conn);
    println!("=>> Genesis state loaded.");
}

// Execute db command with flags
// Input    flags   (flags for db command)
pub fn database_flags(flags: Vec<String>){
    let flag = &flags[0];
    match &flag[..]{
        "-r" => {
            let conn = database::connect_db();
            let target = &flags[1];
            match &target[..]{
                "all" => {
                    println!("=>> Are you sure you want to drop everything? (y/n)");
                    let yn: bool = helper::read_yn();
                    if yn {
                        drop_all();
                        process::exit(1);
                    }
                },
                "account"   => account::drop_account_table(&conn),
                "profile"   => {
                    profile::deactivate(&conn);
                    profile::drop_profile_table(&conn);
                },
                "log"       => log::drop_log_table(&conn),
                "state"     => state::drop_state_table(&conn),
                _           => println!("=>> Unrecognized flag target for [db -drop]"),
            };
            database::close_db(conn);
        },
        _ => println!("=>> Unrecognized flags for command [db]"),
    }
}

//Execute profile command with flags
// Input    flags   (flags for profile command)
pub fn profile_flags(flags: Vec<String>){
    if flags.len() == 0 {
        println!("=>> Profile command requires flags.");
    } else {
        let flag = &flags[0];
        match &flag[..]{
            "-n"    => new_profile(),
            _       => println!("=>> Unrecognized flags for command [profile]"),
        }
    }
}

// Loads and initializes a blank log
pub fn load_code(){
    println!("=>> Which log: a, b, c?");
    let log_choice = helper::read_in();

    println!("=>> How much initial fuel?");
    let fuel_choice = helper::read_in();
    let fuel: i64 = fuel_choice.parse().unwrap();

    let mut l = demo::get_demo_log(&log_choice, fuel);
    // let mess = l.code.clone();

    // TODO Implement in the future
    // let raw_sk = prof.secret_key.clone();
    // let sk = krypto::decode_sk(&raw_sk);
    //
    // let sig =  krypto::sign_message(mess.as_bytes(), &sk).unwrap();
    // l.sig = krypto::
    //
    // database::close_db(conn);

    // Load into database under current database
    let conn = database::connect_db();
    let prof = profile::get_active(&conn).unwrap();
    let acc = account::get_account(&prof.account, &conn);
    l.state = "".to_string();
    l.nonce = acc.log_nonce;
    l.origin = acc.address;
    // Not worrying about sig at the moment.
    log::save_log(l, &conn);
    database::close_db(conn);
}

// Creates a new profile.
pub fn new_profile(){
    let conn = database::connect_db();
    println!("\n=>> Enter the name of new profile:");
    let n = helper::read_in();

    //TODO: Change from static IP to one that the user enters
    println!("=>> Enter the IP address and port (ex. 127.0.0.1:8888):");
    let ip = helper::read_in();
    // let ip = "127.0.0.1:8888";
    profile::new_profile(&n, &ip, &conn);

    //TODO: Profile can fail.
    println!("=>> Profile created.");
    database::close_db(conn);
}

// Main Entry Function
// Connects to network and starts consensus loop
pub fn turbo(){
    println!("\n\n=>> Performing network check...");
    let conn = database::connect_db();
    let p = profile::get_active(&conn).unwrap();

    let trusted: Vec<String> = helper::slice_to_vec(&p.trusted);
    let local_ip: String = p.ip;

    println!("=>> Starting local server...");

    // Initializing Arcs
    // Local Status. String<Status>
    let local_stat: Arc<RwLock<(String, String)>> = Arc::new(RwLock::new((String::new(), String::new())));
    // Connected Nodes and their current status. HashMap<Address, (State, Nonce)>
    let thread_stat: Arc<RwLock<HashMap<String, tenv::tenv>>> = Arc::new(RwLock::new(HashMap::new()));
    // Current Accounts. HashMap<Address, Account>
    // let curr_accs: Arc<RwLock<HashMap<String, account::account>>> = Arc::new(RwLock::new(HashMap::new()));
    // Current Logs. HashMap<Hash, Log>
    let curr_logs: Arc<RwLock<HashMap<String, log::log>>> = Arc::new(RwLock::new(HashMap::new()));

    // Appending previous logs that were not included in the previous state
    let miss_logs = log::get_no_state_logs(&conn);
    for l in miss_logs {
        let larc = curr_logs.clone();
        let mut l_write = larc.write().unwrap();
        let hash = l.hash.clone();
        (*l_write).insert(hash, l);
    }

    database::close_db(conn);

    // Cloning to move into server
    let m_stat = local_stat.clone();
    let t_stat = thread_stat.clone();
    let c_logs = curr_logs.clone();
    //Starting Server
    let _ = thread::spawn(move ||
        server::listen(local_ip, m_stat, t_stat, c_logs)
    );
    //Connecting to trusted accounts for active profile.
    println!("\n=>> There are {:?} trusted accounts on this profile.", trusted.len());
    thread::sleep(Duration::from_millis(500)); // Allow server to bind
    // Connecting to peers
    for ip in trusted{
        server::connect(&ip, local_stat.clone(), thread_stat.clone(), curr_logs.clone());
    }
    let tenv_arc = thread_stat.clone();
    //Starts consensus loop
    println!("=>> Starting Consensus Protocol");
    consensus::consensus_loop(local_stat, tenv_arc, curr_logs);
}

// TODO: Experimental Feature
pub fn coding(){
    let code: String = helper::read_in();
    let conn = database::connect_db();
    let prof = profile::get_active(&conn).unwrap();
    let acc = account::get_account(&prof.account, &conn);
    let mut l = demo::get_demo_log("a", 10000);
    l.state = "".to_string();
    l.nonce = acc.log_nonce;
    l.origin = acc.address;
    l.code = code;
    log::save_log(l, &conn);
    database::close_db(conn);
}
