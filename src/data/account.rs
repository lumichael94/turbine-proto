extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use data::sidechain::sidechain;

pub struct account {
    address     : [u8; 30],  //  id hash of transaction
    t_nonce     : i64,  //  cryptographic nonce
    sig         : [u8; 30],                 //  Modify with Electrum style signatures
    fuel_level  : i64,
    fuel_limit  : i64,
    code        : [u8; 30],                 // TODO: Implement sprint 4
    sidechains  : [u8; 30],       //list of current minting chains
}
