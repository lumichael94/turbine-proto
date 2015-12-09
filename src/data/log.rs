extern crate rand;
extern crate crypto;
extern crate secp256k1;
extern crate rustc_serialize;
extern crate bincode;
extern crate postgres;
extern crate chrono;

use self::secp256k1::*;
use postgres::Connection;
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable};
use std::collections::HashMap;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub struct log {
    pub hash:   String,     //  hash hash of opCodes executed
    pub state:  String,     //  hash of the state state
    pub nonce:  i64,
    pub origin: String,     //  origin account address
    pub target: String,     //  target account address
    pub fuel:   i64,        //  fuel of log (positive or negative fuel)
    pub code:   String,
    pub sig:    Vec<u8>,    //  Modify with Electrum style signatures
}

// Retreives an log.
// Input    hash    Hash of log to retrieve.
// Input    conn    Database connection.
// Output   log     Retrieved log struct.
pub fn get_log (hash : &str, conn: &Connection) -> log{
    let maybe_stmt = conn.prepare("SELECT * FROM log WHERE hash = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let i: String = hash.to_string();
    let rows = stmt.query(&[&i]).unwrap();
    let row = rows.get(0);

    log {
        hash    :   row.get(0),
        state   :   row.get(1),
        nonce   :   row.get(2),
        origin  :   row.get(3),
        target  :   row.get(4),
        fuel    :   row.get(5),
        code    :   row.get(6),
        sig     :   row.get(7),
    }
}

// Saves log struct
// Input    l               Log struct to save.
// Input    conn            Database connection.
pub fn save_log (l : log, conn: &Connection){
    let hash: String = (*l.hash).to_string();
    let state: String = (*l.state).to_string();
    let nonce = &l.nonce;
    let origin : String = (*l.origin).to_string();
    let target : String = (*l.target).to_string();
    let fuel = &l.fuel;
    let code : String = (*l.code).to_string();
    let sig: Vec<u8> = l.sig;
    conn.execute("INSERT INTO log \
                 (hash, state, nonce, origin, target, fuel, code, sig) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                  &[&hash, &state, &nonce, &origin, &target, &fuel, &code, &sig]).unwrap();
}

// Drops specified log
// Input    hash    Hash of log to retrieve.
// Input    conn        Database connection.
pub fn remove_log (hash : &str, conn: &Connection){
    conn.execute("DELETE FROM log WHERE hash = $1", &[&(hash.to_string())]).unwrap();
}

// Creates an log table.
// Input    conn    Database connection.
pub fn create_log_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS log (
                  hash      text,
                  state     text,
                  nonce     bigint,
                  origin    text,
                  target    text,
                  fuel      bigint,
                  code      text,
                  sig       bytea
                  )", &[]).unwrap();
}

// Drops an account table.
// Input    conn    Database connection.
pub fn drop_log_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS log", &[]).unwrap();
}

// Retrieve logs without a state.
// Input    conn    Database connection.
pub fn get_no_state_logs(conn: &Connection)-> Vec<log>{
    let maybe_stmt = conn.prepare("SELECT * FROM log WHERE state = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let i: String = "".to_string();
    let mut logs: Vec<log> = Vec::new();
    for row in stmt.query(&[&i]).unwrap(){
        let l = log {
            hash    :   row.get(0),
            state   :   row.get(1),
            nonce   :   row.get(2),
            origin  :   row.get(3),
            target  :   row.get(4),
            fuel    :   row.get(5),
            code    :   row.get(6),
            sig     :   row.get(7),
        };
        logs.push(l);
    }
    return logs;
}

// Converts log struct to byte vector.
// Input    l           Log struct to convert.
// Output   Vec<u8>     Converted byte vector.
pub fn log_to_vec(l: &log)-> Vec<u8>{
    encode(l, SizeLimit::Infinite).unwrap()
}

// Converts byte vector to account log.
// Input    raw_l     Byte vector to convert.
// Output   log         Converted log.
pub fn vec_to_log(raw_l: &Vec<u8>) -> log{
    let l: log = decode(&raw_l[..]).unwrap();
    return l;
}

// Converts hashmap of logs to byte vector.
// Input    hmap        Hashmap to convert.
// Output   Vec<u8>     Converted byte vector.
pub fn hmap_to_vec(hmap: HashMap<String, log>)-> Vec<u8>{
    let mut log_vec: Vec<String> = Vec::new();
    for (_, l) in hmap{
        let byte_vec: Vec<u8> = log_to_vec(&l.clone());
        let str_vec: String = String::from_utf8(byte_vec).unwrap();
        log_vec.push(str_vec);
    }
    encode(&log_vec, SizeLimit::Infinite).unwrap()
}

// Converts byte vector to hashmap of logs.
// Input    Vec<u8>     Byte vector to convert.
// Output   HashMap     Converted hashmap of logs.
pub fn vec_to_hmap(raw_logs: &Vec<u8>)-> HashMap<String, log>{
    let log_vec: Vec<String> = decode(&raw_logs[..]).unwrap();
    let mut hmap: HashMap<String, log> = HashMap::new();
    for l_string in log_vec{
        let byte_vec: Vec<u8> = l_string.into_bytes();
        let l: log = vec_to_log(&byte_vec);
        let hash = l.clone().hash;
        hmap.insert(hash, l);
    }
    return hmap;
}


// #[cfg(test)]
// mod test {
//     extern crate postgres;
//     use data::database;
//     use postgres::{Connection, SslMode};
//     use super::*;
//
//     #[test]
//     fn test_store_log(){
//         let conn = database::connect_db();
//         create_log_table(&conn);
//
//         let l = log{    hash:       "hash".to_string(),
//                         state:      "state".to_string(),
//                         nonce:      872635,
//                         origin:     "origin".to_string(),
//                         target:     "target".to_string(),
//                         fuel:       567890,
//                         sig:        "signature".to_string(),
//                         proof:      "proof".to_string(),};
//
//         save_log(&l, &conn);
//
//         let retrieved_log: log = get_log(&l.hash, &conn);
//         println!("The retrieved log hash: {:?}", retrieved_log.hash);
//
//         drop_log_table(&conn);
//         database::close_db(conn);
//     }
//
// }
