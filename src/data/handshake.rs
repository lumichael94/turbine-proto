extern crate rand;
extern crate crypto;
extern crate secp256k1;
extern crate rustc_serialize;
extern crate bincode;
extern crate postgres;
extern crate chrono;

use util::*;
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::{encode, decode};
use postgres::{Connection, SslMode};
use rustc_serialize::{Encodable};
use data::{account, database};

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub struct handshake {
    pub status:     String,
    pub address:    String, //  Account Address
    pub account:    account::account,
    pub s_hash:     String,
    pub s_nonce:    i64,
}

// Converts handshake struct to byte vector.
// Input    hs          Handshake struct to convert.
// Output   Vec<u8>     Converted byte vector.
pub fn hs_to_vec(hs: &handshake)-> Vec<u8>{
    encode(hs, SizeLimit::Infinite).unwrap()
}

// Converts byte vector to handshake struct.
// Input    Vec<u8>     Raw hanndshake to convert.
// Output   hs          Converted handshake struct.
pub fn vec_to_hs(raw_hs: &Vec<u8>) -> handshake{
    let hs: handshake = decode(&raw_hs[..]).unwrap();
    return hs;
}

// pub fn check_handshake(raw_hs: Vec<u8>)-> Option<handshake>{
//     let conn: Connection = database::connect_db();
//     let node_hs = vec_to_hs(&raw_hs);
//     let return_hs = node_hs.clone();
//     let node_acc = node_hs.account;
//     let add = node_hs.address;
//
//     // Comparing the local version of the node accounts and what is broadcasted
//     let local_acc = account::get_account(&add, &conn);
//     if local_acc.
//
//     database::close_db(conn);
//     Some(return_hs)
// }
