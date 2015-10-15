extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use data::log::log;
use data::sidechain::sidechain;

pub struct block {
    //  id hash of transaction decided on by trusted peers
    //  Need to compute this yourself and compare to make sure
    //  However, light clients do not need to do so
    //  TODO: Decide on hash function for determining ID
    id              :   [u8; 30],
    parent_id       :   [u8; 30],
    chain_id        :   [u8; 30],
    time_stamp      :   String,    //time of commit
    nonce           :   i64,                //  cryptographic nonce
    logs_hash       :   [u8; 30],
    logs            :   [log],
}

fn init_block(){

}

fn drop_block(){

}
