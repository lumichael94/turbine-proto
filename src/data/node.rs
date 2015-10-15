mod node;

extern crate rand;
extern crate rust-crypto;
extern crate rustc-serialize;
extern crate postgres;
extern crate time;
mod account;

use std::os
use std::sync

struct node {
    ip          : String                //IPv4 or IPv5, main identifier
    address     : [account.address]
    public_key  : [u8]
    status      : String
    trusted     : bool
    blacklisted : bool
    descriptor  : String
}

fn add_node(){

}

fn drop_node(){

}

fn trust_node(){

}

fn untrust_node(){

}

fn blacklist_node(){

}

//Remove nodes that are timed-out, turned-off, or badly connected
fn cleanup_nodes(){

}

fn new_node_db(){

}

fn drop_node_db(){

}

fn last_ping(){

}

fn update_last_ping(){

}

fn last_pong(){

}

fn update_last_pong(){

}
