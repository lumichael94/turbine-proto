mod header_chain;

extern crate rand;
extern crate rust-crypto;
extern crate rustc-serialize;
extern crate postgres;
extern crate time;

use std::os
use std::sync

use sidechain;

struct h_chain {
    id              :   [u8]
    time_stamp      :   chrono::DateTime    //  time of last block commit
    nonce           :   i64                 //  latest block
    last_block_id   :   [u8]
    sidechains      :   [sidechain.id]      //  list of sidechains
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
