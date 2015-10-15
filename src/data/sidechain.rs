mod sidechain;

extern crate rand;
extern crate rust-crypto;
extern crate rustc-serialize;
extern crate postgres;
extern crate time;

use std::os
use std::sync

struct sidechain {
    id              :   [u8]
    parent_id       :   [u8]
    time_stamp      :   chrono::DateTime    //time of commit
    nonce           :   i64     //  cryptographic nonce
    t_list_hash     :   [u8]
    t_list          :   [transaction]
}

fn new_header_chain(){

}

fn drop_header_chain(){

}

//  Sync transactions with another chain. Usually the header chain
fn sync_chain(){

}
