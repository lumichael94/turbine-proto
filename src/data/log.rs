extern crate rand;
extern crate crypto;
extern crate secp256k1;
extern crate rustc_serialize;
extern crate bincode;
extern crate postgres;
extern crate chrono;

use self::secp256k1::*;
use self::secp256k1::key::*;
use postgres::{Connection, SslMode};
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Json, Encoder};

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
pub struct log {
    pub hash:   String,     //  hash hash of opCodes executed
    pub block:  String,     //  hash of the block state
    pub nonce:  i64,
    pub origin: String,     //  origin account address
    pub target: String,     //  target account address
    pub fuel:   i64,        //  fuel of log (positive or negative fuel)
    pub sig:    String,     //  Modify with Electrum style signatures
    pub proof:  String,     //  Proof of the code executed
}

// pub fn new_log (block_hash: &str, log_hash: &str, origin_address: &str, signature: &str, proof: &str) -> log{
//     log{    hash      :   log_hash.to_string(),
//             block   :   block_hash.to_string(),
//             nonce   :   0,
//             origin  :   origin_address.to_string(),
//             fuel    :   0,
//             sig     :   signature.to_string(),
//             proof :   proof.to_string(),
//         }
// }

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
        block   :   row.get(1),
        nonce   :   row.get(2),
        origin  :   row.get(3),
        target  :   row.get(4),
        fuel    :   row.get(5),
        sig     :   row.get(6),
        proof   :   row.get(7),
    }
}

pub fn save_log (l : &log, conn: &Connection){
    let hash: String = (*l.hash).to_string();
    let block: String = (*l.block).to_string();
    let nonce = &l.nonce;
    let origin : String = (*l.origin).to_string();
    let target : String = (*l.target).to_string();
    let fuel = &l.fuel;
    let sig: String = (*l.sig).to_string();
    let proof: String = (*l.proof).to_string();

    conn.execute("INSERT INTO log \
                 (hash, block, nonce, origin, target, fuel, sig, proof) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                  &[&hash, &block, &nonce, &origin, &target, &fuel, &sig, &proof]).unwrap();
}

pub fn remove_log (hash : &str, conn: &Connection){
    conn.execute("DELETE FROM log WHERE hash = $1", &[&(hash.to_string())]).unwrap();
}

pub fn create_log_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS log (
                  hash      text,
                  block     text,
                  nonce     bigint,
                  origin    text,
                  target    text,
                  fuel      bigint,
                  sig       text,
                  proof     text
                  )", &[]).unwrap();
}

pub fn drop_log_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS log", &[]).unwrap();
}

pub fn log_to_vec(l: &log)-> Vec<u8>{
    encode(l, SizeLimit::Infinite).unwrap()
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
//                         block:      "block".to_string(),
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
