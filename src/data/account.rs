extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use self::rand::{Rng, OsRng};
use data::sidechain::sidechain;
use postgres::{Connection, SslMode};

//Make static mut Conn object?

pub struct account{
    pub address     : String,
    pub ip          : String,
    pub trusted     : bool,
    pub t_nonce     : i64,      //  cryptographic nonce, represents number of logs from account
    pub fuel_level  : i64,
    // fuel_limit  : i64,
    // code        : [u8; 30], // TODO: Implement sprint 4
    pub sidechain   : String, //list of current minting chains

}

pub fn create_new_account(sidechain_add: &str, ip_add: &str) -> account{
    let new_address = gen_account_address();
    account{    address:    new_address,
                ip:         ip_add.to_string(),
                trusted:    false,
                t_nonce:    0 as i64,
                fuel_level: 0 as i64,
                sidechain:  sidechain_add.to_string(),}
}

pub fn delete_account(address: String, conn: &Connection){
    conn.execute("DELETE FROM account \
                  WHERE address = $1",
                  &[&address])
                  .unwrap();
}

pub fn store_account(acc: &account, conn: &Connection){
    let add: String = (*acc.address).to_string();
    let ip_add: String = (*acc.ip).to_string();
    let is_trusted: bool = acc.trusted;
    let nonce = acc.t_nonce;
    let fuel = acc.fuel_level;
    let side_add: String = (*acc.sidechain).to_string();

    conn.execute("INSERT INTO account \
                  (address, ip, trusted, t_nonce, fuel_level, sidechain) \
                  VALUES ($1, $2, $3, $4, $5, $6)",
                  &[&add, &ip_add, &is_trusted, &nonce, &fuel, &side_add]).unwrap();
}

pub fn setup_account_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS account (
                    address         text,
                    ip              text,
                    trusted         BOOL,
                    t_nonce         bigint,
                    fuel_level      bigint,
                    sidechain       text
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
        t_nonce     : row.get(3),
        fuel_level  : row.get(4),
        sidechain   : row.get(5),
    }
}

pub fn gen_account_address()-> String{
    let mut rng = match rand::os::OsRng::new(){
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS Rng: {}", e)
    };
    let buf: String = rng.gen_ascii_chars().take(30).collect();
    return buf;
}
