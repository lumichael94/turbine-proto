mod account;

extern crate rand;
extern crate rust-crypto;
extern crate rustc-serialize;
extern crate postgres;
extern crate time;

use std::os
use std::sync

struct account {
    address     : [u8]  //  id hash of transaction
    t_nonce     : i64   //  cryptographic nonce
    sig         : [u8]  //  Modify with Electrum style signatures
    fuel_level  : i64
    fuel_limit  : i64
    code        : [u8]  // TODO: Implement sprint 4
    sidechains  : [sidechain.id]    //list of current minting chains
}
