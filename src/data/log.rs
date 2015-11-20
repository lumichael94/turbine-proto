extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;
use postgres::{Connection, SslMode};

// use std::os;
// use std::sync;

pub struct log {
    pub id      :   String,     //  id hash of transaction
    pub block   :   String,
    pub nonce   :   i64,        //  cryptographic nonce
    pub origin  :   String,     //  origin account address
    pub target  :   String,     //  target account address
    pub fuel    :   i64,        //  fuel of log (positive or negative fuel)
    pub sig     :   String,     //  Modify with Electrum style signatures
}

pub fn new_log (block_id: &str, log_id: &str, origin_address: &str, target_address: &str, signature: &str) -> log{
    log{    id      :   log_id.to_string(),
            block   :   block_id.to_string(),
            nonce   :   0,
            origin  :   origin_address.to_string(),
            target  :   target_address.to_string(),
            fuel    :   0,
            sig     :   signature.to_string()
        }
}

pub fn get_log (id : &str, conn: &Connection) -> log{
    let maybe_stmt = conn.prepare("SELECT * FROM log WHERE id = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let i: String = id.to_string();

    let rows = stmt.query(&[&i]).unwrap();
    let row = rows.get(0);
    let asdf: String = row.get(0);
    // log {
    //     id      :   row.get(0),
    //     block   :   row.get(1),
    //     nonce   :   row.get(2),
    //     origin  :   row.get(3),
    //     target  :   row.get(4),
    //     fuel    :   row.get(5),
    //     sig     :   row.get(6),
    // }

    log {
        id      :   row.get(0),
        block   :   row.get(1),
        nonce   :   1,
        origin  :   "row.get(3)".to_string(),
        target  :   "row.get(4)".to_string(),
        fuel    :   1,
        sig     :   "".to_string(),
    }
}

pub fn store_log (l : &log, conn: &Connection){
    let id: String = (*l.id).to_string();
    let block: String = (*l.block).to_string();
    let nonce = &l.nonce;
    let origin : String = (*l.origin).to_string();
    let target : String = (*l.target).to_string();
    let fuel = &l.fuel;
    let sig: String = (*l.sig).to_string();
    conn.execute("INSERT INTO log \
                 (id, block, nonce, origin, target, fuel, sig) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
                  &[&id, &block, &nonce, &origin, &target, &fuel, &sig]).unwrap();
}

pub fn remove_log (id : &str, conn: &Connection){

    conn.execute("DELETE FROM log WHERE id = $1", &[&(id.to_string())]).unwrap();
}

pub fn setup_log_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS log (
                  id        text,
                  block     text,
                  nonce     bigint,
                  origin    text,
                  target    text,
                  fuel      bigint,
                  sig       text
                  )", &[]).unwrap();
}

pub fn drop_log_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS log", &[]).unwrap();
}
