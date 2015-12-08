use std::thread;
use std::time::Duration;
use std::io::BufRead;
use data::{account, state, database, log, profile};
use util::helper;
use postgres::Connection;
use engine::commands::*;

//====================================================================
// Main Entry Functions
//====================================================================

// Main Initialization Function
pub fn init(){
    println!("\n=>> Welcome to Turbine");
    thread::sleep(Duration::from_millis(500));
    check_db();
}

// Main Endpoint Function
pub fn end(){
    let conn = database::connect_db();
    profile::deactivate(&conn);
    database::close_db(conn);
}

// Command Line Interface.
pub fn command_loop(){
    println!("=>> Starting Command REPL");
    let mut go: bool = true;
    while go {
        go = read_command();
    }
}

// Checks local database tables, initializes profiles.
pub fn check_db(){
    println!("\n=>> Performing database check...");
    println!("=>> Connecting to database...");
    let conn: Connection = database::connect_db();

    println!("=>> Checking profile database...");
    let missing_tables = database::check_tables(&conn);
    if missing_tables.len() != 0{
        for t in missing_tables{
            println!("\n=>> Missing table: {:?}. Creating...", t);
            let _ = match &t[..] {
                "account"   => account::create_account_table(&conn),
                "state"     => state::create_state_table(&conn),
                "log"       => log::create_log_table(&conn),
                "profile"   => profile::create_profile_table(&conn),
                _           => {},
            };
        }
    }
    loop {
        //If there are no profiles, create one.
        if profile::num_profile(&conn) == 0{
            println!("=>> No profiles found. Creating one...");
            new_profile();
            break;
        } else {
            println!("\n=>> Enter name of profile to activate: ");
            let name: String = helper::read_in();
            if profile::activate(&name, &conn) {break;}
        }
    }
    //If there are no states, load Genesis.
    if state::num_states(&conn) == 0{
        println!("=>> No saved states.");
        load_genesis(true);
    }
    database::close_db(conn);
}

// Help command. Displays commands and flags.
pub fn help(){
    let help_text = "\n\t\t\t\tTurbine\n\nGeneral Usage:\n
    profile\t-n \t\t \tCreate a new profile.
    drop\t([table]| all)\t\tRemoves select table or database.
    genesis\t \t\t \tDrops database. Loads Genesis state.
    turbo\t-c, -d\t\t\tStarts turbine in connected/detached mode.
    coding\t \t\t \tWrite opcodes.
    load\t<fuel>\t\t\tLoads the active account with fuel.
    connect\t<ip>\t\t\tConnect to an IP address.\n\nRunning Usage:\n
    account\t\t\t\tShows details of active account.
    block\t\t\t\tShow details of current block.\n";

    println!("{}", help_text);
}

// Reads and executes a command
// Output: Boolean (Success or Failure when reading command)
pub fn read_command() -> bool{
    let response: String = helper::read_in();
    let split = response.split(" ");
    let raw_vec = split.collect::<Vec<&str>>();
    let mut flags = helper::vec_slice_to_string(&raw_vec);
    let command: String = flags.remove(0);
    let _ = match &command[..]{
        "profile"       => profile_flags(flags),
        "db"            => database_flags(flags),
        "genesis"       => load_genesis(false),
        "coding"        => coding(),
        "turbo"         => {thread::spawn(move ||turbo());},
        "help"          => help(),
        "load"          => load_code(),
        "quit"|"exit"   => return false,
        _               => println!("=>> Did not recognize command, please try again."),
    };
    return true;
}
