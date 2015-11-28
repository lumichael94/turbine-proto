extern crate rand;
extern crate crypto;
extern crate postgres;
extern crate chrono;
extern crate bincode;

use std::iter::IntoIterator;
use postgres::{Connection, SslMode};
use util::*;
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Json, Encoder};
use data::account;

pub struct local{
    pub name        : String,   //Name of profile, determined by user
    pub active     : bool,     //Determines if it is the account in use
    pub account     : String,   //Account address
    pub ip          : String,   //Server will bind to this address
    pub secret_key  : Vec<u8>,
}

pub fn drop_local(name: String, conn: &Connection){
    conn.execute("DELETE FROM local \
                  WHERE name = $1",
                  &[&name])
                  .unwrap();
}

pub fn save_local(loc: &local, conn: &Connection){
    let name: String = (*loc.name).to_string();
    let curr: bool = loc.active;
    let acc: String = (*loc.account).to_string();
    let ip: String = (*loc.ip).to_string();
    let ref sk = *loc.secret_key;

    conn.execute("INSERT INTO loc \
                  (name, active, account, ip, secret_key) \
                  VALUES ($1, $2, $3, $4, $5)",
                  &[&name, &curr, &acc, &ip, &sk]).unwrap();
}

pub fn create_local_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS account (
                    name            text,
                    active         bool,
                    account         text,
                    ip              text,
                    secret_key      bytea
                  )", &[]).unwrap();
}

pub fn drop_local_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS local", &[]).unwrap();
}

// Returns the number of local accounts
pub fn num_local(conn: &Connection) -> i32{
    let maybe_stmt = conn.prepare("SELECT * FROM local");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[]).unwrap();
    return rows.len() as i32;
}

pub fn get_local(name: &str, conn: &Connection) -> local{
    let maybe_stmt = conn.prepare("SELECT * FROM local WHERE name = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    
    let n: String = name.to_string();
    let rows = stmt.query(&[&n]).unwrap();
    let row = rows.get(0);
    local {
        name        : row.get(0),
        active      : row.get(1),
        account     : row.get(2),
        ip          : row.get(3),
        secret_key  : row.get(4),
    }
}

// Retrieves the active local profile
pub fn get_active(conn: &Connection) -> local {
    let maybe_stmt = conn.prepare("SELECT * FROM local WHERE active = true");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[]).unwrap();
    let row = rows.get(0);
    local {
        name        : row.get(0),
        active      : row.get(1),
        account     : row.get(2),
        ip          : row.get(3),
        secret_key  : row.get(4),
    }
}

// Switches active profile.
pub fn switch_active(n: &str, conn: &Connection) -> local{
    let mut old_active = get_active(conn);
    old_active.active = false;
    save_local(&old_active, conn);

    let mut new_active = get_local(n, conn);
    new_active.active = true;
    save_local(&new_active, conn);

    return new_active;
}

// Creates a new local profile and new corresponding account
pub fn new_local(n: &str, ip: &str, conn: &Connection) -> local{
    let secret_key = krypto::gen_secret_key();
    let public_key = krypto::gen_public_key(&secret_key);
    let pk: Vec<u8> = encode(&public_key, SizeLimit::Infinite).unwrap();
    let sk: Vec<u8> = encode(&secret_key, SizeLimit::Infinite).unwrap();

    let acc = account::new_local_account(ip, pk, conn);
    account::save_account(&acc, conn);

    local {
        name        : n.to_string(),
        active      : false,
        account     : acc.address,
        ip          : ip.to_string(),
        secret_key  : sk,
    }
}
