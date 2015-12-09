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
use data::handshake::handshake;
use data::account::account;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub struct tenv {
    pub t_stat:     String,     // Thread's Status
    pub n_stat:     String,     // Node's Status
    pub n_add:      String,     // Node's Account Hash
    pub n_state:    String,     // Node's State Hash
    pub n_nonce:    i64,        // Node's State Nonce
}

// Converts handshake struct to thread environment struct.
// Input    hs          Handshake struct to convert.
// Output   tenv        Converted tenv struct.
pub fn hs_to_tenv(hs: handshake) -> tenv{
    let node_add = hs.address;
    let node_acc = hs.account;
    tenv{
        t_stat:     "LISTEN".to_string(),
        n_stat:     hs.status,
        n_add:      node_add.clone(),
        n_state:    node_acc.state,
        n_nonce:    node_acc.s_nonce,
    }
}

// Converts tenv struct to byte vector.
// Input    te          Thread environment struct to convert.
// Output   Vec<u8>     Converted byte vector.
pub fn tenv_to_vec(te: &tenv)-> Vec<u8>{
    encode(te, SizeLimit::Infinite).unwrap()
}

// Converts byte vector to tenv struct.
// Input    raw_tenv    Byte vector to convert.
// Output   tenv        Converted thread environment struct.
pub fn vec_to_tenv(raw_tenv: &Vec<u8>)-> tenv{
    let te: tenv = decode(&raw_tenv[..]).unwrap();
    return te;
}

// TODO: Remove
// pub fn node_to_tenv(te: tenv)-> Vec<u8>{
//     encode(&te, SizeLimit::Infinite).unwrap()
// }
