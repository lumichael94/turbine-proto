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
use std::io::{Read, Write};
use util::helper;
use postgres::{Connection, SslMode};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

//====================================================================
//GENERAL PROTOCOL FUNCTIONS
//====================================================================

//Initial Peer Connections
//Connects to a list of nodes. Appends connected nodes to list in active profile.
pub fn connect_to_peers(addresses: Vec<String>, to_main: Sender<String>,
                        arc: Arc<Mutex<Vec<Sender<String>>>>){
    println!("Connecting to peers...");
    for address in addresses{
        let conn_attempt = server::connect(&address, to_main.clone());
        match conn_attempt {
            None => {},
            Some(to_threads) => {
                arc.lock().unwrap().push(to_threads);
            },
        }
    }
}

//Closing all active connections
//TODO: Implement
pub fn close_connections(from_threads: Receiver<String>, arc: Arc<Mutex<Vec<Sender<String>>>>){
    //Added clone at the end to prevent extended locking.
    let mut to_threads = arc.lock().unwrap().clone();
    let n_threads = to_threads.len();
    loop {
        //Loop until all nodes are disconnected.
        let to_thread = to_threads.pop().unwrap();
        let _ = to_thread.send("quit".to_string());
        let counter = to_threads.len();
        println!("Size of arc is: {:?}", counter);
        if  counter == 0 {break};
    }
}

//Sending functions
pub fn send_handshake(stream: &mut TcpStream, conn: &Connection){
    // Retrieving Personal Account
    let my_acc: account::account = account::get_active_account(conn);
    // Sending account for verification
    send_account(stream, my_acc.address, conn);
}

pub fn send_account(stream :&mut TcpStream, address: String, conn: &Connection){
    let acc = account::get_account(&address, conn);
    let buf = &account::acc_to_vec(&acc);
    let _ = stream.write(&[4, buf.len() as u8]);
    let _ = stream.write(buf);

}

pub fn send_log(stream :&mut TcpStream, hash: String, conn: &Connection){
    let l = log::get_log(&hash, &conn);
    let buf = &log::log_to_vec(&l);
    let _ = stream.write(&[6, buf.len() as u8]);
    let _ = stream.write(buf);
}

pub fn send_state(stream :&mut TcpStream, hash: String, conn: &Connection){
    let s = state::get_state(&hash, conn);
    let buf = &state::state_to_vec(&s);
    let _ = stream.write(&[8, buf.len() as u8]);
    let _ = stream.write(buf);
}

pub fn request_logs(stream: &mut TcpStream, state_hash: String){
    let raw_address: &[u8] = state_hash.as_bytes();
    let size = raw_address.len();
    let _ = stream.write(&[5, size as u8]);
    let _ = stream.write(raw_address);
}

//TODO: Update connected nodes. Remove duplicates from Arc.
pub fn update_connected(){

}
