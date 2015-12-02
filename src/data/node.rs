extern crate rand;
extern crate crypto;
extern crate secp256k1;
extern crate rustc_serialize;
extern crate bincode;
extern crate postgres;
extern crate chrono;

use rustc_serialize::{Encodable};


#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub struct node {
    pub status:     String,     // Node's Status
    pub t_status:   String,     // Thread's Status
    pub acc_hash:   String,     // Node's Account Hash
    pub s_hash:     String,     // Node's State Hash
    pub s_nonce:    String,     // Node's State Nonce
}
