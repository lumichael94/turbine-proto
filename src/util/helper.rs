extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Json, Encoder};
// use rustc_serialize::json::Json
use std::fs::File;
use std::io::Read;
use data::account;
use postgres::{Connection, SslMode};

// Please don't judge me for this. I'm tired.
// pub fn format_code(text: &str) -> Vec<String>{
pub fn slice_to_vec(text: &str) -> Vec<String>{
    let s: String = text.to_string();
    let split = s.split(",");
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
