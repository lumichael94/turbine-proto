extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;

use network::{server, proto};
use data::{account, state, database, log};
use vm::env;
use util::{helper, krypto};
use postgres::{Connection, SslMode};
use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};

// Checking Functions
//=====================================

pub fn check_handshake(raw_acc: Vec<u8>, ) -> bool{
    check_account(raw_acc)
}


// pub fn check_state(raw_s: Vec<u8>) -> bool{
//     let conn: Connection = database::connect_db();
//
//     let s = state::vec_to_state(raw_s);
//     let local_s = state::get_state(&s.hash, &conn);
//
//     database::close_db(conn);
// }


pub fn check_account(raw_acc: Vec<u8>) -> bool{
    let conn: Connection = database::connect_db();

    let node_acc = account::vec_to_acc(raw_acc);
    let node_state_hash: String = node_acc.state;

    //TODO: If node_state is above the local state, then the function will fail. Fix.
    let node_state = state::get_state(&node_state_hash, &conn);

    let local_state = state::get_current_state(&conn);

    if local_state.nonce -3 > node_state.nonce{
        database::close_db(conn);
        return false;
    } else {
        let local_acc = account::get_account(&node_acc.address, &conn);
        if node_acc.log_nonce < local_acc.log_nonce{
            return false;
        }
        database::close_db(conn);
        return true;
    }
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
