extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use network::server;
use data::{account, database};
use std::io::Read;
use std::io::Write;
use util::helper;
use postgres::{Connection, SslMode};

pub fn connect_to_peers(){
    println!("Connecting to peers.");
    let peer: &str = "127.0.0.1:8888";
    server::connect(peer);
}

pub fn send_handshake(mut stream :&mut TcpStream){
    let conn = database::connect_db();
    // Retrieving Personal Account
    let my_acc: account::account = account::get_current_account(&conn);
    // Sending account for verification
    send_account(stream, &my_acc);
    database::close_db(conn);
}

pub fn send_account(mut stream :&mut TcpStream, acc: &(account::account)){
    // let acc: account::account = account::get_account(&add, conn);
    let buf = &account::acc_to_vec(acc);
    let _ = stream.write(&[3, buf.len() as u8]);
    let _ = stream.write(buf);
}

pub fn send_log(){

}

pub fn send_block(){

}

pub fn request_block(){

}

pub fn request_log(){

}

// pub enum proto_code {
//
//
// }
