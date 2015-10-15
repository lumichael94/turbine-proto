extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;

use data::sidechain::sidechain;

pub struct Hchain {
    id              :   [u8; 30],
    time_stamp      :   String,    //  time of last block commit
    nonce           :   i64,                 //  latest block
    last_block_id   :   [u8; 30],
    sidechains      :   [u8; 30],      //  list of sidechains
}

fn new_header_chain(){

}

fn drop_header_chain(){

}

fn commit_block(){

}

fn spawn_sidechain(){

}

fn destroy_sidechain(){

}

fn sync_chain(){

}
