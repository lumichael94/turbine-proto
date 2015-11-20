extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;

// Please don't judge me for this. I'm tired.
pub fn format_code(text: &str) -> Vec<String>{
    let mut s: String = text.to_string();
    let mut split = s.split(",");
    let coll = split.collect::<Vec<&str>>();
    return vec_slice_to_string(&coll);
}

pub fn vec_slice_to_string(v: &Vec<&str>) -> Vec<String>{
    let mut vec: Vec<String> = Vec::new();
    for x in v {
        vec.push(x.to_string());
    }
    return vec;
}

fn chain_count(){

}

fn block_fuel(){

}

pub fn node_count(){

}

pub fn ping_node(){

}

pub fn node_latency(){

}
