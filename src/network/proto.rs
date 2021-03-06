extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::net::TcpStream;
use data::{account, log, state, tenv, handshake as hs};
use std::io::{Read, Write};
use postgres::Connection;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

//====================================================================
// GENERAL PROTOCOL FUNCTIONS
//====================================================================

// Initiate Handshake. Handshake also updates the node struct.
// Input    stream          TcpStream with connected node.
// Input    local_stat      Status of the main thread
// Input    tenv_stat       Status of the connected threads
// Output   Option<tenv>    Returns thread environment struct
pub fn request_handshake(stream: &mut TcpStream, conn: &Connection, local_stat: Arc<RwLock<(String, String)>>,
    tenv_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>) -> Option<tenv::tenv>{
        // Requesting handshake
        let _ = stream.write(&[2, 0]);
        let mut buf = [0; 2];
        let _ = stream.read(&mut buf);
        let last_state = state::get_current_state(conn);
        // If no response, try twice more, then fail.
        for _ in 0..2 {
            if buf[0] == 2 { // Node also starts off with request handshake.
                buf = [0; 2];
                let arc = local_stat.clone();
                let main_tup = arc.read().unwrap();
                send_handshake(stream, conn, main_tup.0.clone());

                let _ = stream.read(&mut buf); // Reading again for the transmission size
                let raw_hs = read_stream(stream, buf[1]);

                let node_hs = hs::vec_to_hs(&raw_hs);
                let their_nonce: i64 = node_hs.s_nonce;
                let last_nonce: i64 = last_state.nonce; // Local State nonce
                let thread_stat : String;

                if last_nonce == their_nonce{           // Local node is in sync
                    thread_stat = "SYNCED".to_string();
                } else if last_nonce > their_nonce {    // Local node is more updated
                    thread_stat = "AHEAD".to_string();
                } else {                                // Local node is behind
                    thread_stat = "BEHIND".to_string();
                }
                let te = tenv::tenv{
                    t_stat:     thread_stat,
                    n_stat:     node_hs.status,
                    n_add:      node_hs.address,
                    n_state:    node_hs.s_hash,
                    n_nonce:    node_hs.s_nonce,
                };
                let add = te.n_add.clone();
                // Appending node into node status arc.
                let hs_arc = tenv_stat.clone();
                let hs_te = te.clone();
                (*hs_arc).write().unwrap().insert(add, hs_te);
                return Some(te);
            }
        }
    return None;
}

// Sends handshake struct
// Input    stream          TcpStream with connected node.
// Input    conn            Database connection.
pub fn send_handshake(stream: &mut TcpStream, conn: &Connection, local_stat: String){
    let my_acc: account::account = account::get_active_account(conn);
    let hs_acc = my_acc.clone(); // Clone for sending

    let send_hs = hs::handshake {
        status:  local_stat,
        address: my_acc.address,
        s_hash:  my_acc.state,
        s_nonce: my_acc.s_nonce,
        account: hs_acc,
    };
    let send_buf = &hs::hs_to_vec(&send_hs);
    let _ = stream.write(&[3, send_buf.len() as u8]);
    let _ = stream.write(send_buf);
}

// Sends account struct
// Input    stream          TcpStream with connected node.
// Input    conn            Database connection.
pub fn send_account(stream :&mut TcpStream, address: String, conn: &Connection){
    let acc = account::get_account(&address, conn);
    let buf = &account::acc_to_vec(&acc);
    let _ = stream.write(&[4, buf.len() as u8]);
    let _ = stream.write(buf);
}

// Sends log struct
// Input    stream          TcpStream with connected node.
// Input    conn            Database connection.
pub fn send_log(stream :&mut TcpStream, hash: String, conn: &Connection){
    let l = log::get_log(&hash, &conn);
    let buf = &log::log_to_vec(&l);
    let _ = stream.write(&[6, buf.len() as u8]);
    let _ = stream.write(buf);
}

// Sends state struct
// Input    stream          TcpStream with connected node.
// Input    conn            Database connection.
pub fn send_state(stream :&mut TcpStream, hash: String, conn: &Connection){
    let s = state::get_state(&hash, conn);
    let buf = &state::state_to_vec(&s);
    let _ = stream.write(&[8, buf.len() as u8]);
    let _ = stream.write(buf);
}

// Requesting logs for a committed state
// Input    stream          TcpStream with connected node.
// Input    curr_logs       Collection of the current uncommitted logs.
// Input    s_hash            Requested state hash.
pub fn request_logs(stream: &mut TcpStream, curr_logs: Arc<RwLock<HashMap<String, log::log>>>, s_hash: String){
    let raw_shash = s_hash.as_bytes();
    let size = raw_shash.len();
    let _ = stream.write(&[13, size as u8]);    // Identifying opCodes
    let _ = stream.write(&raw_shash);           // Sends hash of target state

    let mut incoming = [0;2];
    let _ = stream.read(&mut incoming).unwrap();
    let raw_logs = read_stream(stream, incoming[1]);
    let hmap: HashMap<String, log::log> = log::vec_to_hmap(&raw_logs);
    let log_arc = curr_logs.clone();
    let mut log_map = log_arc.write().unwrap();

    for (l_hash, l) in hmap{
        if !log_map.contains_key(&l_hash){
            let save_l = l.clone();
            (*log_map).insert(l_hash, save_l);
        }
    }
}

// Requests the next state after specified state.
// Input    stream          TcpStream with connected node.
// Input    s_hash          State hash before requested state.
// Output   state           Requested state
pub fn request_state_after(stream: &mut TcpStream, s_hash: String) -> state::state{
    let raw_shash = s_hash.as_bytes();
    let size = raw_shash.len();

    let _ = stream.write(&[15, size as u8]);
    let mut incoming = [0;2];
    let _ = stream.read(&mut incoming).unwrap();
    let raw_state = read_stream(stream, incoming[1]);
    state::vec_to_state(raw_state)
}

//====================================================================
// PROPOSING FUNCTIONS
// Contains functions called during phase: "proposing"
//====================================================================

// Sending possible state hash
// Input    stream          TcpStream with connected node.
// Input    s_hash          State hash before requested state.
pub fn send_poss_state_hash(stream: &mut TcpStream, s_hash: String){
    let raw_hash = s_hash.as_bytes();
    let size = raw_hash.len();
    let _ = stream.write(&[13, size as u8]);
    let _ = stream.write(raw_hash);
}

// Requesting possible state hash
// Input    stream          TcpStream with connected node.
// Output   String          Connected nodes proposal state hash
pub fn request_poss_shash(stream: &mut TcpStream)-> String{
    // Requesting Possible State Hash
    let _ = stream.write(&[13, 0]);
    let mut incoming = [0;2];
    let _ = stream.read(&mut incoming).unwrap();
    String::from_utf8(read_stream(stream, incoming[1])).unwrap()
}

// Requesting possible logs to be included in the current state
// Input    stream          TcpStream with connected node.
pub fn request_poss_logs(stream: &mut TcpStream)->HashMap<String, log::log>{
    // Requesting Logs
    // let _ = stream.write(&[4, 0]);
    let mut incoming = [0;2];
    let _ = stream.read(&mut incoming);
    let raw_logs = read_stream(stream, incoming[1]);
    log::vec_to_hmap(&raw_logs)
}

// Sends possible logs to be included in the current state
// Input    stream          TcpStream with connected node.
// Input    log_hmap        Hashmap from log hash -> log struct. Current uncommitted logs.
pub fn send_poss_logs(stream: &mut TcpStream, log_hmap: HashMap<String, log::log>){
    // Sending Logs
    let send_logs = log::hmap_to_vec(log_hmap);
    let size = send_logs.len();
    let _ = stream.write(&[14, size as u8]);
    let _ = stream.write(&send_logs);
}

// Exchange current accounts of uncommitted state
// Input    stream      TcpStream with connected node.
// Input    acc_hmap    Local current uncommitted accounts
// Output   HashMap     Connected node's current uncommitted accounts.
pub fn exchange_accounts(stream: &mut TcpStream, acc_hmap: HashMap<String, account::account>
)->HashMap<String, account::account>{
    // Requesting Logs
    let _ = stream.write(&[13, 0]);
    let mut incoming = [0;2];
    let _ = stream.read(&mut incoming);
    let raw_accs = read_stream(stream, incoming[2]);
    // Sending Logs
    let send_accs = account::hmap_to_vec(acc_hmap);
    let size = send_accs.len();
    let _ = stream.write(&[14, size as u8]);
    let _ = stream.write(&send_accs);
    account::vec_to_hmap(&raw_accs)
}

// Compare and append missing logs from connected node.
// Input    node_logs   Connected node's current uncommitted accounts.
// Input    logs_arc    Local current uncommitted accounts
// Output   i32         Number of missing logs.
pub fn compare_logs(node_logs: HashMap<String, log::log>, logs_arc: Arc<RwLock<HashMap<String, log::log>>>)->i32{
    let our_logs: HashMap<String, log::log> = logs_arc.read().unwrap().clone();
    let my_log = our_logs.clone();
    let their_logs: HashMap<String, log::log> = node_logs;
    let mut counter = 0;
    // Iterate and save missing logs
    let mut l_arc = logs_arc.write().unwrap();
    for (l_hash, l) in their_logs{
        if !my_log.contains_key(&l_hash){
            let save_l = l.clone();
            let hash = l.hash;
            (*l_arc).insert(hash, save_l);
            counter += 1;
        }
    }
    return counter;
}

// Update node struct and thread status.
// Input    stream      TcpStream with connected node.
// Input    conn        Database connection.
// Output   tenv        Status of connected node (tenv struct)
pub fn request_update(stream: &mut TcpStream, conn: &Connection)-> tenv::tenv{

    let mut buf = [0;2];
    // let _ = stream.write(&[4,0]);    // Requesting Update
    let _ = stream.read(&mut buf);
    let raw_tenv: Vec<u8> = read_stream(stream, buf[1]);
    let mut te: tenv::tenv = tenv::vec_to_tenv(&raw_tenv);

    // Changing thread status
    let their_nonce: i64 = te.n_nonce;
    let last_state: state::state = state::get_current_state(conn);
    let last_nonce: i64 = last_state.nonce;     // Local State nonce

    if last_nonce == their_nonce{               // Local node is in sync
        te.t_stat = "SYNCED".to_string();
    } else if last_nonce > their_nonce {        // Local node is more updated
        te.t_stat = "AHEAD".to_string();
    } else {                                    // Local node is behind
        te.t_stat = "BEHIND".to_string();
    }
    return te;
}
// Sends updated local status
// Input    stream      TcpStream with connected node.
// Input    conn        Database connection.
// Input    local_stat  Status of the main thread.
pub fn send_update(stream: &mut TcpStream, conn: &Connection, local_stat: String){
    let my_acc: account::account = account::get_active_account(conn);
    let te = tenv::tenv {
        t_stat:     "".to_string(), // No need to transfer
        n_stat:     local_stat,
        n_add:      my_acc.address,
        n_state:    my_acc.state,
        n_nonce:    my_acc.s_nonce,
    };
    let send_buf = &tenv::tenv_to_vec(&te);
    let _ = stream.write(&[5, send_buf.len() as u8]);
    let _ = stream.write(send_buf);
}

// Reads data from TcpStream
// Input    stream      TcpStream with connected node.
// Input    length      Length of message received.
// Output   Vec<u8>     Message received.
pub fn read_stream(stream: &mut TcpStream, length: u8) -> Vec<u8>{
	let mut data_buf = vec![0; length as usize];
	let _ = stream.read(&mut data_buf[..]);
	return data_buf;
}

// Retrieve status from main thread.
// Input    local_stat  Status of the main thread
// Output   Tuple       Current status of the main thread and current state.
pub fn get_local_stat(local_stat: Arc<RwLock<(String, String)>>)->(String, String){
    let marc = local_stat.clone();
    let reader = marc.read().unwrap();
    let status: String = reader.0.clone();
    let state: String = reader.1.clone();
    drop(reader);
    return (status, state);
}

// TODO: Not used. Remove.
// Compare and trade missing accounts with connected node.
// pub fn compare_accounts(stream: &mut TcpStream, node_accs: HashMap<String, account::account>,
// accs_arc: Arc<RwLock<HashMap<String, account::account>>>){
//     let our_accs: HashMap<String, account::account> = accs_arc.read().unwrap().clone();
//     let my_accs = our_accs.clone();
//     let mut their_accs: HashMap<String, account::account> = node_accs;
//     let mut send_accs: HashMap<String, account::account> = HashMap::new();
//
//     for (add, acc) in our_accs{
//         if !their_accs.contains_key(&add){
//             send_accs.insert(add, acc);
//         } else {
//             let _ = their_accs.remove(&add);
//         }
//     }
//     // Iterate and save missing accounts
//     let mut accs_arc = accs_arc.write().unwrap();
//     for (add, acc) in their_accs{
//         if !my_accs.contains_key(&add){
//             let save_acc = acc.clone();
//             let add = acc.address;
//             (*accs_arc).insert(add, save_acc);
//         }
//     }
//     // Send accounts
//     let size = send_accs.len();
//     let _ = stream.write(&[15, size as u8]);
//     let raw_accs: Vec<u8> = account::hmap_to_vec(send_accs);
//     let _ = stream.write(&raw_accs[..]);
//
// }
