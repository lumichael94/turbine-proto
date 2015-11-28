extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use network::server;
use data::{account, database, log, state};
use std::io::Read;
use std::io::Write;
use util::helper;
use postgres::{Connection, SslMode};
use std::sync::mpsc::{self, Sender, Receiver};

//General functions
//Connects to a list of nodes. Appends connected nodes to list in active profile.
pub fn connect_to_peers(addresses: Vec<String>, to_thread: Sender<String>) -> Vec<String>{
    println!("Connecting to peers...");
    //Connected peers list
    let mut connected: Vec<String> = Vec::new();
    for address in addresses{
        let tx = to_thread.clone();
        let c: bool = server::connect(&address, tx);
        if c == true {
            connected.push(address);
        }
    }
    return connected;
}

//Closing all active connections
//TODO: Implement
pub fn close_connections(addresses: Vec<String>){

}

//Sending functions
pub fn send_handshake(stream :&mut TcpStream){
    let conn = database::connect_db();
    // Retrieving Personal Account
    let my_acc: account::account = account::get_active_account(&conn);
    // Sending account for verification
    send_account(stream, my_acc.address);
    database::close_db(conn);
}

pub fn send_account(stream :&mut TcpStream, address: String){
    let conn = database::connect_db();
    let acc = account::get_account(&address, &conn);
    let buf = &account::acc_to_vec(&acc);
    let _ = stream.write(&[3, buf.len() as u8]);
    let _ = stream.write(buf);

    database::close_db(conn);
}

pub fn send_log(stream :&mut TcpStream, hash: String){
    let conn = database::connect_db();
    let l = log::get_log(&hash, &conn);
    let buf = &log::log_to_vec(&l);
    let _ = stream.write(&[5, buf.len() as u8]);
    let _ = stream.write(buf);
    database::close_db(conn);
}

pub fn send_state(stream :&mut TcpStream, hash: String){
    let conn = database::connect_db();
    let s = state::get_state(&hash, &conn);
    let buf = &state::state_to_vec(&s);
    let _ = stream.write(&[8, buf.len() as u8]);
    let _ = stream.write(buf);
    database::close_db(conn);
}

pub fn request_logs(stream: &mut TcpStream, state_hash: String){
    let conn = database::connect_db();

    let raw_address: &[u8] = state_hash.as_bytes();
    let size = raw_address.len();
    let _ = stream.write(&[5, size as u8]);
    let _ = stream.write(raw_address);

    database::close_db(conn);
}

//TODO: Update connected nodes
pub fn update_connected(){

}
