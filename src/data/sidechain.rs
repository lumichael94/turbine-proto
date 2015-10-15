extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use data::log::log;

pub struct sidechain {
    id              :   [u8; 30],
    parent_id       :   [u8; 30],
    time_stamp      :   String,    //time of commit
    nonce           :   i64,     //  cryptographic nonce
    t_list_hash     :   [u8; 30],
    t_list          :   [log],
}

fn new_header_chain(){

}

fn drop_header_chain(){

}

//  Sync transactions with another chain. Usually the header chain
fn sync_chain(){

}
