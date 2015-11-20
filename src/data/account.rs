extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
// use self::rand::{Rng, OsRng};
use postgres::{Connection, SslMode};

pub struct account{
    pub address     : String,
    pub ip          : String,
    pub trusted     : bool,
    pub log_nonce   : i64,
    pub fuel        : i64,
    pub code        : String,
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

    conn.execute("INSERT INTO account \
                  (address, ip, trusted, log_nonce, fuel, code) \
                  VALUES ($1, $2, $3, $4, $5, $6)",
                  &[&add, &ip_add, &is_trusted, &nonce, &fuel, &code]).unwrap();
}

pub fn create_account_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS account (
                    address         text,
                    ip              text,
                    trusted         BOOL,
                    log_nonce       bigint,
                    fuel            bigint,
                    code            text
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
    }
}

// pub fn gen_account_address()-> String{
//     let mut rng = match rand::os::OsRng::new(){
//         Ok(g) => g,
//         Err(e) => panic!("Failed to obtain OS Rng: {}", e)
//     };
//     let buf: String = rng.gen_ascii_chars().take(32).collect();
//     return buf;
// }
