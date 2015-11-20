extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use self::rand::{Rng, OsRng};
use postgres::{Connection, SslMode};

use data::log::log;

pub struct state {
    nonce           :   i64,
    hash            :   String,
    prev_block      :   String,     //Hash of previous block
    time_stamp      :   String,
    log_hash        :   String,
    proof_hash      :   String,
    fuel_exp        :   i64,
}

pub fn save_state(s: &state, conn: &Connection){
    let nonce: i64 = s.nonce;
    let hash: String = (*s.hash).to_string();
    let prev_block: String = (*s.prev_block).to_string();
    let time_stamp: String = (*s.time_stamp).to_string();
    let log_hash: String = (*s.log_hash).to_string();
    let proof_hash: String = (*s.proof_hash).to_string();
    let fuel_exp: i64 = s.fuel_exp;

    conn.execute("INSERT INTO state \
                  (nonce, hash, prev_block, time_stamp, log_hash, proof_hash, fuel_exp) \
                  VALUES ($1, $2, $3, $4, $5, $6, $7)",
                  &[&nonce, &hash, &prev_block, &time_stamp, &log_hash, &proof_hash, &fuel_exp]).unwrap();
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
        prev_block: row.get(2),
        time_stamp: row.get(3),
        log_hash:   row.get(4),
        proof_hash: row.get(5),
        fuel_exp:   row.get(6),
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
                    prev_block      text,
                    time_stamp      text,
                    log_hash        text,
                    proof_hash      text,
                    fuel_exp        BIGINT
                  )", &[]).unwrap();
}

pub fn drop_state_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS state", &[]).unwrap();
}

// Tests
#[cfg(test)]
mod test {
  use std::net;
  use std::thread;
  use super::*;
  use postgres::{Connection, SslMode};
  use data::database;


  #[test]
  fn test_state() {
    println!("State test");
    let conn = database::connect_db();
    create_state_table(&conn);

    let s: state = state{   nonce:      123987,
                            hash:       "hash".to_string(),
                            prev_block: "prev hash".to_string(),
                            time_stamp: "timestamp".to_string(),
                            log_hash:   "log hash".to_string(),
                            proof_hash: "proof hash".to_string(),
                            fuel_exp:   15208,};

    save_state(&s, &conn);
    let retrieved_s = get_state(&s.hash, &conn);
    println!("State hash is: {:?}", retrieved_s.hash);
    // drop_state_table(&conn);
    database::close_db(conn);
  }
}
