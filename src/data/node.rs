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
use rustc_serialize::{Encodable};


#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub struct node {
    pub status:     String,     // Node's Status
    pub t_status:   String,     // Thread's Status
    pub acc_hash:   String,     // Node's Account Hash
    pub s_hash:     String,     // Node's State Hash
    pub s_nonce:    String,     // Node's State Nonce
}

pub fn node_to_vec(nde: &node)-> Vec<u8>{
    encode(nde, SizeLimit::Infinite).unwrap()
}

pub fn vec_to_node(raw_nde: &Vec<u8>) -> node{
    let nde: node = decode(&raw_nde[..]).unwrap();
    return nde;
}
