extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;
extern crate regex;

use std::{os, sync, net, thread, io};
use network::{server, proto};
use data::{account, state, database, log, profile};
use vm::env;
use util::{helper, krypto};
use self::postgres::{Connection, SslMode};
use std::io::BufRead;
use self::regex::Regex;
use main::consensus;

pub fn init() -> Vec<String>{
    println!("Initiating Turbine.");
    check_db();
    check_net()
}

pub fn main() {
    let connected = main_loop(&mut init());
    end(connected);
}

//TODO: Implement this main loop
pub fn main_loop(connected: &mut Vec<String>) -> Vec<String>{
    return Vec::new();
}

pub fn end(connected: Vec<String>) {
    let conn = database::connect_db();
    //TODO: Close all connections and end threads.
    proto::close_connections(connected);

    //TODO: Deactivate current profile.
    profile::deactivate(&conn);
}

pub fn check_db(){
    println!("Performing database check...");
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
        println!("Enter name of profile to activate: ");
        let name: String = read_in();
        profile::activate(&name, &conn);
    }
    database::close_db(conn);
}

pub fn check_net() -> Vec<String> {
    let conn = database::connect_db();
    let p = profile::get_active(&conn).unwrap();
    let trusted: Vec<String> = helper::slice_to_vec(&p.trusted);
    let local_ip: String = p.ip;

    println!("Performing network check...");
    println!("Starting local server...");
    //Starting Server
    let _ = thread::spawn(move|| server::listen(&local_ip));

    //Connecting to trusted accounts for active profile.
    println!("There are {:?} trusted accounts on this profile.", trusted.len());
    let connected: Vec<String> = proto::connect_to_peers(trusted);
    println!("Connected to {:?} peers.", connected.len());

    // TODO CHECK: Does the server stall when you don't join threads?
    // let _ = server_thread.join();
    database::close_db(conn);
    return connected;
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
    println!("Are you sure you want to drop everything?");
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
    let flag = &flags[0];
    match &flag[..]{
        "-n"    => new_profile(),
        _       => println!("Unrecognized flags for command [profile]"),
    }
}

//Creates a new profile.
pub fn new_profile(){
    let conn = database::connect_db();
    println!("Enter the name of the profile:");
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
    return response;
}

//Reads response to yes or no prompt.
pub fn read_yn() -> bool{
    let response: String = read_in();
    let yn = match &response[..] {
                "y"|"Y"|"yes"|"Yes"|"YES"   => true,
                "n"|"N"|"no"|"No"|"NO"      => false,
                _                           => {
                                                    println!("Try again: ");
                                                    return read_yn();
                                                },
            };
    return yn;
}

//Reads and executes a command
pub fn read_command(){
    let response: String = read_in();
    let split = response.split(" ");
    let raw_vec = split.collect::<Vec<&str>>();
    let mut flags = helper::vec_slice_to_string(&raw_vec);
    let command: String = flags.remove(0);

    let _ = match &command[..]{
        "profile"   => profile_flags(flags),
        "db"        => database_flags(flags),
        _   => {
                    println!("Did not recognize command, please try again.");
                    read_command();
            },
    };
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_main() {
      println!("Beginning test...");
      main();
  }
}
