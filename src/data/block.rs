extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use data::log::log;
use data::sidechain::sidechain;

pub struct block {
    //  id hash of transaction decided on by trusted peers
    //  Need to compute this yourself and compare to make sure
    //  However, light clients do not need to do so
    //  TODO: Decide on hash function for determining ID
    id              :   String,
    parent_id       :   String,
    chain_id        :   String,
    time_stamp      :   String,    //time of commit
    nonce           :   i64,                //  cryptographic nonce
    logs_hash       :   String,
    logs            :   [log],
}


//no real idea what I'm doing here

fn init_block(){
    // conn.execute("INSERT INTO account \
    //               (address, t_nonce, fuel_level, sidechain) \
    //               VALUES ($1, $2, $3, $4)",
    //               &[&account.address,
    //                 &account.t_nonce,
    //                 &account.fuel_level,
    //                 &account.sidechain])
    //         .unwrap();
}

fn drop_block(){
    // conn.execute("DELETE FROM account \
    //               WHERE address == $1",
    //               &[&address])
    //         .unwrap();
}


pub fn setup_block_table(){
    //not sure how to handle this junk
    // conn = Connection::connect("postgres://postgres@localhost", &SslMode::None)
    //     .unwrap();
    //
    // conn.execute("CREATE TABLE block (
    //                 id              bytea,
    //                 parent_id       bytea,
    //                 chain_id        bytea,
    //                 time_stamp      varchar,
    //                 nonce           bigint
    //                 logs_hash       bytea,
    //                 logs            bytea
    //               )", &[]).unwrap();
}
