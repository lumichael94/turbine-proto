extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;
extern crate regex;

use std::{os, sync, net, thread, io};
use network::{server, proto};
use data::{account, state, database, log, local};
use vm::env;
use util::{helper, krypto};
use self::postgres::{Connection, SslMode};
use std::io::BufRead;
use self::regex::Regex;

pub fn init() -> Connection{
    println!("Initiating Turbine.");
    let conn = check_db();
    // check_net();

    //TCP Server
    let server_thread = thread::spawn(move||{
                            server::listen("127.0.0.1:8888");
                        });

    let client_thread = thread::spawn(move ||{
                            proto::connect_to_peers();
                        });
    let _ = server_thread.join();
    let _ = client_thread.join();
    return conn;
}

// pub fn main() {
//     println!("Hello World!");
//     let conn: Connection = init();
//     end(conn);
// }

pub fn end(conn: Connection) {
    database::close_db(conn);
}

pub fn check_db() -> Connection{
    println!("Executing database check.");
    println!("Connecting to database...");
    let conn: Connection = database::connect_db();

    println!("Checking local database...");
    let missing_tables = database::check_tables(&conn);
    if missing_tables.len() != 0{
        for t in missing_tables{
            println!("Missing table: {:?}. Creating...", t);

            // Converting t to &'a str
            let _ = match &t[..] {
                "account"   => account::create_account_table(&conn),
                "state"     => state::create_state_table(&conn),
                "log"       => log::create_log_table(&conn),
                "local"     => local::create_local_table(&conn),
                _           => {},
            };
        }
    }

    //If there are no local profiles, create one.
    if local::num_local(&conn) == 0{
        println!("No profiles found. Creating one...");
        new_profile();
    }
    return conn;
}

pub fn check_net(){
    //Check how many nodes we have in the database.
    //Check how many of those nodes are online.
    //Connecting...
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
        local::drop_local_table(&conn);
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
    println!("Enter the IP address and port (ex. 127.0.0.1:8888):");
    let ip = read_in();
    local::new_local(&n, &ip, &conn);

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
