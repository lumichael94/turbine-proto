extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use self::rand::{Rng, OsRng};
use data::sidechain::sidechain;

pub struct account {
    //address     : [u8; 20], //  id hash of transaction
    // address     : &'a Vec<u8>,
    address     : Vec<u8>,
    t_nonce     : i64,      //  cryptographic nonce, represents number of logs from account
    fuel_level  : i64,
    // fuel_limit  : i64,
    // code        : [u8; 30], // TODO: Implement sprint 4
    sidechain   : Vec<u8>, //list of current minting chains
}

pub fn create_new_account(sidechain_add: &Vec<u8>) -> account{
    let new_address = gen_account_address();
    account{    address: new_address,
                t_nonce: 0 as i64,
                fuel_level: 0 as i64,
                sidechain: sidechain_add.to_vec(),}
}

pub fn destroy_account(address: [u8; 30]){

}

pub fn store_account(acc: account){

}

pub fn gen_account_address() -> Vec<u8>{
    let address = rand::thread_rng().gen_iter::<u8>().take(20).collect::<Vec<u8>>();
    return address;

}
