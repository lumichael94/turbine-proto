use std::os;
use std::sync;

use network::{server, proto};
use data::{account, state, database, log};
use vm::env;
use util::{helper, krypto};
use postgres::{Connection, SslMode};
use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

//====================================================================
// GENERAL CONSENSUS FUNCTIONS
//====================================================================

// Main consensus function
pub fn consensus_loop(from_threads: Receiver<String>,
    arc:  Arc<Mutex<HashMap<String, Sender<String>>>>){
    let conn = database::connect_db();

    // Listen Loop
    loop {
        update_connected();
        let trusted = arc.clone();

        // Logs and accounts of the current state.
        // Only applies to logs/accounts that were modified/participated.
        let s_logs: Vec<String> = Vec::new();
        let s_accounts: Vec<String> = Vec::new();

        if should_propose(trusted, &conn){break;}
    }
    database::close_db(conn);
}

//====================================================================
// LOCAL NODE PHASE FUNCTIONS
//====================================================================

// Cycles through connected nodes to set update local node status: "listening", "proposing", "committed".
// "listening": Node is accepting all ongoing logs and accounts of a state.
// "proposing": Node is proposing set of logs and accounts. It will only allow modifications
//              if more than 80% of trusted node list form a consensus on accepting a log
//              or account.
// "committed": Node has computed, verified and committed a state from the logs and accounts.
// Status only applies if node is working on its most current state. Otherwise, it is perpetually
// listening.

// Checking whether the local node status should propose
pub fn should_propose(arc: Arc<Mutex<HashMap<String, Sender<String>>>>, conn: &Connection) -> bool{
    // Determine majority state, count
    let mut state_map: HashMap<String, i32> = HashMap::new();
    let nodes: HashMap<String, Sender<String>> = arc.lock().unwrap().clone();
    let n_nodes = nodes.len() as i32;

    for (node_add, _ ) in nodes {
        let node_acc = account::get_account(&node_add, conn);
        let node_state = node_acc.state;
        let contains: bool = state_map.contains_key(&node_state);

        if contains {
            let counter = state_map.get(&node_state).unwrap() + 1;
            state_map.insert(node_state, counter);
        } else {
            state_map.insert(node_state, 0);
        }
    }
    // Find popular state
    let mut max_pop: i32 = 0;
    let mut max_state = String::new();
    for (state_hash, state_pop) in state_map{
        if state_pop > max_pop {
            max_state = state_hash;
            max_pop = state_pop;
        }
    }
    // If maximum population is more than 80% of trusted nodes and node state is equal to
    // the current state, then local phase = "proposing"
    let local_state: String = state::get_current_state(&conn).hash;
    let threshold = 0.8 as i32;
    let percentage = max_pop / n_nodes;
    if (max_state == local_state) && (percentage > threshold){
        return true;
    }
    return false;
}

// Checking whether the local node status should commit the current state
pub fn should_commit(arc: Arc<Mutex<HashMap<String, Sender<String>>>>, conn: &Connection) -> bool{
    return false;
}

// Proposing Phase
pub fn proposing(){}

// Committing Phase
pub fn committing(){}

// OpCodes from thread to consensus loop.
pub fn do_thread_code(thread_code: String){
    let data: Vec<String> = helper::slice_to_vec(&thread_code);
    let code: String = data.get(0).unwrap().to_string();

    match &code[..] {
        // Node arrived at a final state and voting.
        "voting" => {
            println!("Node is voting.");
        },
        // Node has finished voting and committed a state.
        "voted" => {
            println!("Node has voted.");
        },
        "" => {},
        _ =>{},
    }
}



//TODO: Update connected nodes. Remove unresponsive nodes.
pub fn update_connected(){

}

// Checking Functions
//=====================================

// pub fn check_state(raw_s: Vec<u8>) -> bool{
//     let conn: Connection = database::connect_db();
//
//     let s = state::vec_to_state(raw_s);
//     let local_s = state::get_state(&s.hash, &conn);
//
//     database::close_db(conn);
// }

// Checks an account
pub fn check_account(raw_acc: Vec<u8>) -> Option<account::account>{
    let conn: Connection = database::connect_db();
    let node_acc = account::vec_to_acc(&raw_acc);
    let node_address = node_acc.address;
    let node_log_n = node_acc.log_nonce;

    // If account exists locally, compare local and received accounts
    if account::account_exist(&node_address, &conn){
        let local_acc = account::get_account(&node_address, &conn);
        // If node has an outdated nonce, reject the node.
        if node_log_n < local_acc.log_nonce{
            database::close_db(conn);
            return None;
        }
    }

    // TODO: Converting again due to borrowing. Fix
    let return_acc = account::vec_to_acc(&raw_acc);
    account::save_account(&return_acc, &conn);
    database::close_db(conn);
    Some(return_acc)
}

// Update Functions
//=====================================

pub fn update_state(){

}

pub fn update_account(){

}

pub fn update_log(){

}

//Careful on updating log. Logs are immutable unless a mistake has been made.
// pub fn update_log(){
//
// }

//Local VM Functions
pub fn execute_log(){

}

pub fn rollback_state(){

}
