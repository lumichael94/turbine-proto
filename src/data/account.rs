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
use std::fs::File;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
pub struct account{
    pub address     : String,
    pub ip          : String,
    pub trusted     : bool,
    pub log_nonce   : i64,
    pub fuel        : i64,
    pub code        : String,
    pub state       : String,
    pub public_key  : Vec<u8>,
    pub secret_key  : Vec<u8>,
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
    let is_trusted: bool = acc.trusted;
    let nonce = acc.log_nonce;
    let fuel = acc.fuel;
    let code: String = (*acc.code).to_string();
    let state: String = (*acc.state).to_string();
    let ref public_key = *acc.public_key;
    let ref secret_key = *acc.secret_key;

    conn.execute("INSERT INTO account \
                  (address, ip, trusted, log_nonce, fuel, code, state, public_key, secret_key) \
                  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                  &[&add, &ip_add, &is_trusted, &nonce, &fuel, &code, &state, &public_key, &secret_key]).unwrap();
}

pub fn create_account_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS account (
                    address         text,
                    ip              text,
                    trusted         BOOL,
                    log_nonce       bigint,
                    fuel            bigint,
                    code            text,
                    state           text,
                    public_key      bytea,
                    secret_key      bytea
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
        trusted     : row.get(2),
        log_nonce   : row.get(3),
        fuel        : row.get(4),
        code        : row.get(5),
        state       : row.get(6),
        public_key  : row.get(7),
        secret_key  : row.get(8),
    }
}

// // Grabs current account
// pub fn get_current_account(conn: &Connection) -> account {
//     let info_dir = "config.json";
//     let mut file = File::open(info_dir).unwrap();
//     let mut data = String::new();
//     file.read_to_string(&mut data).unwrap();
//     let json = Json::from_str(&data).unwrap();
//     let json_obj = json.find_path(&["local", "accounts"]).unwrap();
//
//     //Current account address
//     let add: String = json_obj.as_string().unwrap().to_string();
//     return get_account(&add, conn)
// }


//TODO: Actually retrieve current account
pub fn get_current_account(conn: &Connection) -> account {
    get_account("my_acc", conn)
}

pub fn acc_to_vec(acc: &account)-> Vec<u8>{
    encode(acc, SizeLimit::Infinite).unwrap()
}

pub fn vec_to_acc(raw_acc: Vec<u8>) -> account{
    let acc: account = decode(&raw_acc[..]).unwrap();
    return acc;
}

pub fn new_account() -> account{
    let secret_key = krypto::gen_secret_key();
    let public_key = krypto::gen_public_key(&secret_key);
    let pk: Vec<u8> = encode(&public_key, SizeLimit::Infinite).unwrap();
    let sk: Vec<u8> = encode(&secret_key, SizeLimit::Infinite).unwrap();

    account {   address:    "my_acc".to_string(),
                ip:         "127.0.0.1:8888".to_string(),
                trusted:    false,
                log_nonce:  0 as i64,
                fuel:       0 as i64,
                code:       "".to_string(),
                state:      "state address".to_string(),        //TODO: Use real state address
                public_key: pk,
                secret_key: sk,
            }
}

pub fn scrub_key(acc: account) -> account{
    let blank_key: Vec<u8> = Vec::new();
    account {   address:    acc.address,
                ip:         acc.ip,
                trusted:    acc.trusted,
                log_nonce:  acc.log_nonce,
                fuel:       acc.fuel,
                code:       acc.code,
                state:      acc.state,
                public_key: acc.public_key,
                secret_key: blank_key,
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
    //                         trusted:    false,
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
