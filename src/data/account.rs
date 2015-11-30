extern crate rand;
extern crate crypto;
extern crate postgres;
extern crate chrono;
extern crate secp256k1;
extern crate rustc_serialize;
extern crate bincode;

use std::os;
use std::sync;
use std::str;
use std::iter::IntoIterator;
use self::secp256k1::*;
use self::secp256k1::key::*;
use postgres::{Connection, SslMode};
use util::*;
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Json, Encoder};
use data::{state, profile, database};

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
pub struct account{
    pub address     : String,
    pub ip          : String,
    pub log_nonce   : i64,
    pub fuel        : i64,
    pub code        : String,
    pub s_nonce     : i64,
    pub state       : String,
    pub public_key  : Vec<u8>,
    pub stack       : String,
    pub memory      : String,
    pub pc          : i64,
}

pub fn drop_account(address: String, conn: &Connection){
    conn.execute("DELETE FROM account \
                  WHERE address = $1",
                  &[&address])
                  .unwrap();
}

pub fn save_account(acc: &account, conn: &Connection){
    let add: String = (*acc.address).to_string();
    let ip_add: String = (*acc.ip).to_string();
    let nonce = acc.log_nonce;
    let fuel = acc.fuel;
    let code: String = (*acc.code).to_string();
    let state_nonce: i64 = acc.s_nonce;
    let state: String = (*acc.state).to_string();
    let ref public_key = *acc.public_key;
    let stack: String = (*acc.stack).to_string();
    let memory: String = (*acc.memory).to_string();
    let pc: i64 = acc.pc;

    conn.execute("INSERT INTO account \
                  (address, ip, log_nonce, fuel, code, s_nonce, state, public_key, stack, memory, pc) \
                  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                  &[&add, &ip_add, &nonce, &fuel, &code, &state_nonce,
                  &state, &public_key, &stack, &memory, &pc]).unwrap();
}

pub fn create_account_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS account (
                    address         text primary key,
                    ip              text,
                    log_nonce       bigint,
                    fuel            bigint,
                    code            text,
                    s_nonce         bigint,
                    state           text,
                    public_key      bytea,
                    stack           text,
                    memory          text,
                    pc              bigint
                  )", &[]).unwrap();
}

pub fn drop_account_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS account", &[]).unwrap();
}

pub fn get_account(add: &str, conn: &Connection) -> account{
    let maybe_stmt = conn.prepare("SELECT * FROM account WHERE address = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let a: String = add.to_string();
    let rows = stmt.query(&[&a]).unwrap();
    let row = rows.get(0);
    account {
        address     : row.get(0),
        ip          : row.get(1),
        log_nonce   : row.get(2),
        fuel        : row.get(3),
        code        : row.get(4),
        s_nonce     : row.get(5),
        state       : row.get(6),
        public_key  : row.get(7),
        stack       : row.get(8),
        memory      : row.get(9),
        pc          : row.get(10),
    }
}

pub fn account_exist(add: &str, conn: &Connection) -> bool{
    let maybe_stmt = conn.prepare("SELECT * FROM account WHERE address = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[&add]);
    match rows {
        Err(_) => false,
        Ok(r) => {
            if r.len() != 0 {
                true
            } else {
                false
            }
        },
    }
}

// Returns the state nonce of an account
pub fn get_state_nonce(address: &str, conn: &Connection) -> i64{
    let acc = get_account(address, conn);
    return acc.s_nonce;
}

// Retrieves the active account
pub fn get_active_account(conn: &Connection) -> account {
    let p = profile::get_active(&conn).unwrap();
    // println!("Got active profile");
    let acc = get_account(&p.account, &conn);
    return acc;
}

pub fn acc_to_vec(acc: &account)-> Vec<u8>{
    encode(acc, SizeLimit::Infinite).unwrap()
}

pub fn vec_to_acc(raw_acc: &Vec<u8>) -> account{
    let acc: account = decode(&raw_acc[..]).unwrap();
    return acc;
}

pub fn new_local_account(ip: &str, pk: Vec<u8>) -> account{
    let add = krypto::gen_string(16);

    //TODO: No current state when first initialized
    account {   address:    add,
                ip:         ip.to_string(),
                log_nonce:  0 as i64,
                fuel:       0 as i64,
                code:       "".to_string(),
                s_nonce:    0 as i64,
                state:      "".to_string(),
                public_key: pk,
                stack:      "".to_string(),
                memory:     "".to_string(),
                pc:         0 as i64,
            }
}

//Tests
// #[cfg(test)]
// mod test {
//     extern crate rand;
//     extern crate postgres;
//
//     use super::*;
//     use data::database;
//     use postgres::{Connection, SslMode};

    // #[test]
    // fn test_store_account(){
    //     let conn = database::connect_db();
    //     create_account_table(&conn);
    //
    //     let ip: &str = "192.168.1.1";
    //     let acc = account { address:    "address".to_string(),
    //                         ip:         "192.168.1.1".to_string(),
    //                         log_nonce:  0 as i64,
    //                         fuel:       0 as i64,
    //                         code:       "".to_string(),
    //                         state:      "state address".to_string(),
    //                         public_key: "public_key".to_string(),
    //                     };
    //
    //     save_account(&acc, &conn);
    //
    //     let a = get_account(&acc.address, &conn);
    //
    //     drop_account_table(&conn);
    //     database::close_db(conn);
    // }
// }
