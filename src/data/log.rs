mod log;

extern crate rand;
extern crate rust-crypto;
extern crate rustc-serialize;
extern crate postgres;
extern crate time;

use std::os
use std::sync

//  This is only a 1-to-1 transaction.
//  TODO: Implement multi-sigs
struct log {
    id      :   String  //  id hash of transaction
    nonce   :   i64     //  cryptographic nonce
    origin  :   [i64]   //  origin account address
    target  :   [i64]   //  target account address
    fuel    :   i64     //  fuel of log (positive or negative fuel)
    sig     :   [i64]   //  Modify with Electrum style signatures
}

//  TODO: Implement transaction receipts. Sprint 4
struct log_receipt {}

fn find_log (){

}

fn store_log (){

}

fn remove_log (){

}
