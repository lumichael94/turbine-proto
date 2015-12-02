extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use network::server;
use data::{account, database, log, state, node, handshake as hs};
use std::io::{Read, Write};
use util::helper;
use postgres::{Connection, SslMode};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use main::consensus;

//====================================================================
// GENERAL PROTOCOL FUNCTIONS
//====================================================================

// Initial Peer Connections
// Connects to a list of nodes. Appends connected nodes to list in active profile.
// TODO: Remove
// pub fn connect_to_peers(addresses: Vec<String>,main_stat: Arc<RwLock<(String, String)>>,
// nodes_stat: Arc<RwLock<HashMap<String, node::node>>>, curr_accs: Arc<RwLock<HashMap<String, account::account>>>,
//     curr_logs: Arc<RwLock<HashMap<String, log::log>>>){
//     println!("Connecting to peers...");
//
// }

//Closing all active connections
//TODO: Fix errors.
pub fn close_connections(from_threads: Receiver<String>, arc: Arc<Mutex<Vec<Sender<String>>>>){
    //Added clone at the end to prevent extended locking.
    let mut to_threads = arc.lock().unwrap().clone();
    loop {
        //Loop until all nodes are disconnected.
        let to_thread = to_threads.pop().unwrap();
        let _ = to_thread.send("quit".to_string());
        let counter = to_threads.len();
        println!("Size of arc is: {:?}", counter);
        if  counter == 0 {break};
    }
}

//Initiate Handshake.
pub fn handshake(stream: &mut TcpStream, conn: &Connection, main_stat: Arc<RwLock<(String, String)>>,
    nodes_stat: Arc<RwLock<HashMap<String, node::node>>>) -> Option<hs::handshake>{

    // Retrieves information to form handshake struct
    let my_acc: account::account = account::get_active_account(conn);
    let mut m_stat = String::new();
    let (m_stat, _) = main_stat.read().unwrap().clone();

    // Clone for sending

    let hs_acc = my_acc.clone();
    let local_hs = hs::handshake {
        status:  m_stat,
        address: my_acc.address,
        s_hash:  my_acc.state,
        s_nonce: my_acc.s_nonce,
        account: hs_acc,
    };

    let buf = &hs::hs_to_vec(&local_hs);

    // Sending node struct
    let _ = stream.write(&[3, buf.len() as u8]);
    let _ = stream.write(buf);

    let mut buffer = [0; 2];
    let _ = stream.read(&mut buffer);

    // If no response, try twice more, then fail.
    for _ in 0..1 {
        // If a node is sending handshake...
        if buffer[0] == 3 {
            let raw_hs = server::read_stream(stream, buffer[1]);
            let node_hs =  hs::vec_to_hs(&raw_hs);
            return Some(node_hs);
        }
    }
        // Pause, try again.
        thread::sleep(Duration::from_millis(500));
        let _ = stream.read(&mut buffer);
    return None;
}

//Sending functions
// pub fn send_handshake(stream: &mut TcpStream, conn: &Connection){
//     // Retrieving Personal Account
//     let my_acc: account::account = account::get_active_account(conn);
//     // Sending account for verification
//     send_account(stream, my_acc.address, conn);
// }

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
