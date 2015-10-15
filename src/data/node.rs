extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use data::account::account;

pub struct node {
    ip          : String,               //IPv4 or IPv5, main identifier
    address     : [u8; 30],
    public_key  : [u8; 30],
    status      : String,
    trusted     : bool,
    blacklisted : bool,
    descriptor  : String,
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
