extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;

//  This is only a 1-to-1 transaction.
//  TODO: Implement multi-sigs
pub struct log {
    id      :   String,  //  id hash of transaction
    nonce   :   i64,     //  cryptographic nonce
    origin  :   [u8; 30],   //  origin account address
    target  :   [u8; 30],   //  target account address
    fuel    :   i64,    //  fuel of log (positive or negative fuel)
    sig     :   [u8; 30],   //  Modify with Electrum style signatures
}

//  TODO: Implement transaction receipts. Sprint 4
// struct log_receipt;

fn new_log (){

}

fn find_log (){

}

fn store_log (){

}

fn remove_log (){

}
