mod block;

extern crate rand;
extern crate rust-crypto;
extern crate rustc-serialize;
extern crate postgres;
extern crate chrono;

mod transaction;

use std::os
use std::sync

struct block {
    //  id hash of transaction decided on by trusted peers
    //  Need to compute this yourself and compare to make sure
    //  However, light clients do not need to do so
    //  TODO: Decide on hash function for determining ID
    id              :   [u8]
    parent_id       :   [u8]
    chain_id        :   [chain.id]
    time_stamp      :   chrono::DateTime    //time of commit
    nonce           :   i64     //  cryptographic nonce
    logs_hash       :   [u8]
    logs            :   [transaction]
}

fn init_block(){

}

fn drop_block(){

}
