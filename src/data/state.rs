extern crate rand;
extern crate crypto;
extern crate secp256k1;
extern crate rustc_serialize;
extern crate bincode;
extern crate postgres;
extern crate chrono;

use postgres::Connection;
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable};

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
pub struct state {
    pub nonce           :   i64,
    pub hash            :   String,
    pub prev_state      :   String,    // Hash of previous state
    pub acc_hash        :   String,    // Hash of accounts
    pub l_hash          :   String,    // Hash of logs
    pub fuel_exp        :   i64,
}

pub fn save_state(s: &state, conn: &Connection){
    let nonce: i64 =    s.nonce;
    let hash: String =  (*s.hash).to_string();
    let acc_hash =      (*s.acc_hash).to_string();
    let l_hash =        (*s.l_hash).to_string();
    let prev_state =    (*s.prev_state).to_string();
    let fuel_exp =      s.fuel_exp;

    conn.execute("INSERT INTO state \
                  (nonce, hash, prev_state, acc_hash, l_hash, fuel_exp) \
                  VALUES ($1, $2, $3, $4, $5, $6)",
                  &[&nonce, &hash, &prev_state, &acc_hash, &l_hash, &fuel_exp]).unwrap();
}

pub fn get_state(hash: &str, conn: &Connection) -> state{
    let maybe_stmt = conn.prepare("SELECT * FROM state WHERE hash = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let h: String = hash.to_string();
    let rows = stmt.query(&[&h]).unwrap();
    let row = rows.get(0);
    state {
        nonce:      row.get(0),
        hash:       row.get(1),
        prev_state: row.get(2),
        acc_hash:   row.get(3),
        l_hash:     row.get(4),
        fuel_exp:   row.get(5),
    }
}

pub fn drop_state(hash: &str, conn: &Connection){
    conn.execute("DELETE FROM state \
                  WHERE hash = $1",
                  &[&(hash.to_string())])
                  .unwrap();
}

pub fn create_state_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS state (
                    nonce           BIGINT,
                    hash            text,
                    prev_state      text,
                    acc_hash        text,
                    l_hash          text,
                    fuel_exp        BIGINT
                  )", &[]).unwrap();
}

pub fn drop_state_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS state", &[]).unwrap();
}

// Returns the number of states
pub fn num_states(conn: &Connection) -> i32{
    let maybe_stmt = conn.prepare("SELECT * FROM state");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err),
    };
    let rows = stmt.query(&[]).unwrap();
    return rows.len() as i32;
}

pub fn get_current_state(conn: &Connection) -> state{
    let maybe_stmt = conn.prepare("SELECT * FROM state WHERE nonce = (select max(nonce) from state);");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };

    let rows = stmt.query(&[]).unwrap();
    let row = rows.get(0);

    state {
            nonce:      row.get(0),
            hash:       row.get(1),
            prev_state: row.get(2),
            acc_hash:   row.get(3),
            l_hash:     row.get(4),
            fuel_exp:   row.get(5),
    }
}

pub fn state_to_vec(s: &state)-> Vec<u8>{
    encode(s, SizeLimit::Infinite).unwrap()
}

pub fn vec_to_state(raw_s: Vec<u8>) -> state{
    let s: state = decode(&raw_s[..]).unwrap();
    return s;
}

// // Tests
// #[cfg(test)]
// mod test {
//   use std::net;
//   use std::thread;
//   use super::*;
//   use postgres::{Connection, SslMode};
//   use data::database;
//
//   #[test]
//   fn test_state() {
//     println!("State test");
//     let conn = database::connect_db();
//     create_state_table(&conn);
//
//     let s: state = state{   nonce:      123987,
//                             hash:       "hash".to_string(),
//                             prev_state: "prev hash".to_string(),
//                             time_stamp: "timestamp".to_string(),
//                             log_hash:   "log hash".to_string(),
//                             proof_hash: "proof hash".to_string(),
//                             fuel_exp:   15208,};
//
//     save_state(&s, &conn);
//     let retrieved_s = get_state(&s.hash, &conn);
//     println!("State hash is: {:?}", retrieved_s.hash);
//     // drop_state_table(&conn);
//     database::close_db(conn);
//   }
// }
