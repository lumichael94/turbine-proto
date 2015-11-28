extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;
extern crate regex;

use std::thread;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use network::{server, proto};
use data::{account, state, database, log, profile};
use util::helper;
use self::postgres::Connection;
use std::io::BufRead;
use std::sync::mpsc::{self, Sender, Receiver};

pub fn init() -> (Receiver<String>, Arc<Mutex<Vec<Sender<String>>>>){
    println!("Initiating Turbine.");
    check_db();
    check_net()
}

pub fn main() {
    let (from_threads, connected) = init();
    main_loop(from_threads, connected);
}

pub fn main_loop(from_threads: Receiver<String>, connected: Arc<Mutex<Vec<Sender<String>>>>){
    println!("\n\nInitiating command REPL");
    let mut go: bool = true;
    while go {
        print!(">> ");
        io::stdout().flush().unwrap();
        go = read_command();
    }
    end(from_threads, connected);
}

pub fn end(from_threads: Receiver<String>, connected: Arc<Mutex<Vec<Sender<String>>>>) {
    //TODO: Close all connections and end threads.

    proto::close_connections(from_threads, connected);

    //TODO: Deactivate current profile.
    let conn = database::connect_db();
    profile::deactivate(&conn);

    //TODO: Remove this.
    loop{}
}

pub fn check_db(){
    println!("\n\nPerforming database check...");
    println!("Connecting to database...");
    let conn: Connection = database::connect_db();

    println!("Checking profile database...");
    let missing_tables = database::check_tables(&conn);
    if missing_tables.len() != 0{
        for t in missing_tables{
            println!("Missing table: {:?}. Creating...", t);
            let _ = match &t[..] {
                "account"   => account::create_account_table(&conn),
                "state"     => state::create_state_table(&conn),
                "log"       => log::create_log_table(&conn),
                "profile"   => profile::create_profile_table(&conn),
                _           => {},
            };
        }
    }

    //If there are no profile profiles, create one.
    if profile::num_profile(&conn) == 0{
        println!("No profiles found. Creating one...");
        new_profile();
    } else {
        println!("\nCreate a new profile? (y/n)");
        let yn: bool = read_yn();
        if yn {
            new_profile();
        } else {
            //TODO: Fails if profile doesn't exist.
            println!("\nEnter name of profile to activate: ");
            let name: String = read_in();
            profile::activate(&name, &conn);
        }
    }
    database::close_db(conn);
}

pub fn check_net() -> (Receiver<String>, Arc<Mutex<Vec<Sender<String>>>>){
    println!("\n\nPerforming network check...");
    let conn = database::connect_db();
    let p = profile::get_active(&conn).unwrap();
    let trusted: Vec<String> = helper::slice_to_vec(&p.trusted);
    let local_ip: String = p.ip;

    println!("Starting local server...");
    //Creating Server Channel
    //to_server sends a kill command
    //to_turbo sends an connnected command
    let (to_main, from_threads): (Sender<String>, Receiver<String>) = mpsc::channel();
    let connected: Arc<Mutex<Vec<Sender<String>>>> = Arc::new(Mutex::new(Vec::new()));
    let serv_arc = connected.clone();
    let serv_to_main = to_main.clone();

    //Starting Server
    let _ = thread::spawn(move ||
        server::listen(local_ip, serv_to_main, serv_arc)
    );

    //Waiting for the server to bind.
    let bound = from_threads.recv().unwrap();
    if bound == "bound".to_string(){
        let conn_arc = connected.clone();
        let threads_to_main = to_main.clone();
        //Connecting to trusted accounts for active profile.
        println!("\nThere are {:?} trusted accounts on this profile.", trusted.len());
        proto::connect_to_peers(trusted, threads_to_main, conn_arc);
        let conn_len = connected.lock().unwrap().len();
        println!("Connected to {:?} peers.", conn_len);
    } else {
        println!("Error binding to address.");
    }
    database::close_db(conn);
    return (from_threads, connected);
}

//Displays commands and flags
pub fn help(){

}

//Commands
//Execute db command with flags
pub fn database_flags(flags: Vec<String>){
    let flag = &flags[0];
    match &flag[..]{
        "-drop" => {
                    let target = &flags[1];
                    match &target[..]{
                            "all" => drop_all(),
                            _     => println!("Unrecognized flag target for [db -drop]"),
                        };
            },
        _       => println!("Unrecognized flags for command [db]"),
    }
}

//Drops all database tables
pub fn drop_all(){
    println!("Are you sure you want to drop everything? (y/n)");
    let yn: bool = read_yn();
    if yn {
        let conn = database::connect_db();
        account::drop_account_table(&conn);
        state::drop_state_table(&conn);
        log::drop_log_table(&conn);
        profile::drop_profile_table(&conn);
        database::close_db(conn);
    }
}

//Execute profile command with flags
pub fn profile_flags(flags: Vec<String>){
    if flags.len() == 0 {
        println!("Profile command requires flags.");
    } else {
        let flag = &flags[0];
        match &flag[..]{
            "-n"    => new_profile(),
            _       => println!("Unrecognized flags for command [profile]"),
        }
    }
}

//Creates a new profile.
pub fn new_profile(){
    let conn = database::connect_db();
    println!("\nEnter the name of the profile:");
    let n = read_in();

    //TODO: Change from static IP to one that the user enters
    // println!("Enter the IP address and port (ex. 127.0.0.1:8888):");
    // let ip = read_in();
    let ip = "127.0.0.1:8888";
    profile::new_profile(&n, &ip, &conn);

    //TODO: Profile can fail.
    println!("Profile created.");
    database::close_db(conn);
}

//User Input Functions
//Reads and returns user response.
pub fn read_in() -> String{
    let stdin = io::stdin();
    let mut response = String::new();
    let _ = stdin.read_line(&mut response);

    //Remove "\n" from response
    let valid = response.len() - 1;
    response.truncate(valid);
    return response;
}

//Reads response to yes or no prompt.
pub fn read_yn() -> bool{
    let response: String = read_in();
    let yn = match &response[..] {
                "y"|"Y"|"yes"|"Yes"|"YES"   => true,
                "n"|"N"|"no"|"No"|"NO"      => false,
                _                           => {
                                                    println!("Invalid response. Try again.");
                                                    return read_yn();
                                                },
            };
    return yn;
}

//Reads and executes a command
pub fn read_command() -> bool{
    let response: String = read_in();
    let split = response.split(" ");
    let raw_vec = split.collect::<Vec<&str>>();
    let mut flags = helper::vec_slice_to_string(&raw_vec);
    let command: String = flags.remove(0);

    let _ = match &command[..]{
        "profile"   => profile_flags(flags),
        "db"        => database_flags(flags),
        "quit"      => {
            return false;
        },
        _           => {
            println!("Did not recognize command, please try again.");
        },
    };
    return true;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_main() {
      println!("Beginning test...");
      main();
    //   drop_all();
  }
}
