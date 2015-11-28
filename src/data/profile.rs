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

pub struct profile{
    pub name        : String,       //Name of profile, determined by user
    pub active      : bool,         //Determines if it is the account in use
    pub account     : String,       //Account address
    pub ip          : String,       //Server will bind to this address
    pub secret_key  : Vec<u8>,
    pub trusted     : String,  //List of trusted accounts
}

pub fn drop_profile(name: String, conn: &Connection){
    conn.execute("DELETE FROM profile \
                  WHERE name = $1",
                  &[&name])
                  .unwrap();
}

pub fn save_profile(loc: &profile, conn: &Connection){

    let name: String = (*loc.name).to_string();
    let curr: bool = loc.active;
    let acc: String = (*loc.account).to_string();
    let ip: String = (*loc.ip).to_string();
    // let trusted: String = (*loc.trusted).to_string();

    //TODO: Change away from hardcoded constants.
    let trusted: String = "127.0.0.1:8888".to_string();
    let ref sk = *loc.secret_key;

    let exist: bool = profile_exist(&name, conn);
    if exist {
        conn.execute("UPDATE profile \
                        SET active = $2, account = $3, ip = $4,\
                         secret_key = $5, trusted = $6 \
                        WHERE name = $1",
                      &[&name, &curr, &acc, &ip, &sk, &trusted]).unwrap();
    } else {
        conn.execute("INSERT INTO profile \
                      (name, active, account, ip, secret_key, trusted) \
                      VALUES ($1, $2, $3, $4, $5, $6)",
                      &[&name, &curr, &acc, &ip, &sk, &trusted]).unwrap();
    }
}

pub fn create_profile_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS profile (
                    name            text primary key,
                    active          bool,
                    account         text,
                    ip              text,
                    secret_key      bytea,
                    trusted         text
                  )", &[]).unwrap();
}

pub fn drop_profile_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS profile", &[]).unwrap();
}

pub fn profile_exist(name: &str, conn: &Connection) -> bool{
    let maybe_stmt = conn.prepare("SELECT * FROM profile WHERE name = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[&name]);
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

// Returns the number of profile accounts
pub fn num_profile(conn: &Connection) -> i32{
    let maybe_stmt = conn.prepare("SELECT * FROM profile");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[]).unwrap();
    return rows.len() as i32;
}

pub fn get_profile(name: &str, conn: &Connection) -> profile{
    let maybe_stmt = conn.prepare("SELECT * FROM profile WHERE name = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };

    let n: String = name.to_string();
    let rows = stmt.query(&[&n]).unwrap();
    let row = rows.get(0);
    profile {
        name        : row.get(0),
        active      : row.get(1),
        account     : row.get(2),
        ip          : row.get(3),
        secret_key  : row.get(4),
        trusted     : row.get(5),
    }
}

// Retrieves the active profile profile
pub fn get_active(conn: &Connection) -> Result<profile, &str> {
    let maybe_stmt = conn.prepare("SELECT * FROM profile WHERE active = true");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[]);
    match rows {
        Err(_) => Err("Error retrieving active profile."),
        Ok(r) => {
            if r.len() != 0 {
                let row = r.get(0);
                let p = profile {
                                    name        : row.get(0),
                                    active      : row.get(1),
                                    account     : row.get(2),
                                    ip          : row.get(3),
                                    secret_key  : row.get(4),
                                    trusted     : row.get(5),
                                };
                Ok(p)
            } else {
                Err("No active profiles.")
            }
        },
    }
}

// Switches active profile.
pub fn switch_active(n: &str, conn: &Connection){
    let possible_active = get_active(conn);
    match possible_active {
        Err(_) => activate(n, conn),
        Ok(mut p) => {
            p.active = false;
            save_profile(&p, conn);
            activate(n, conn);
        },
    }
}

// Creates a new profile profile and new corresponding account
pub fn new_profile(n: &str, ip: &str, conn: &Connection) -> profile{
    let secret_key = krypto::gen_secret_key();
    let public_key = krypto::gen_public_key(&secret_key);
    let pk: Vec<u8> = encode(&public_key, SizeLimit::Infinite).unwrap();
    let sk: Vec<u8> = encode(&secret_key, SizeLimit::Infinite).unwrap();
    let acc = account::new_local_account(ip, pk);
    account::save_account(&acc, conn);

    let p = profile {
        name        : n.to_string(),
        active      : false,
        account     : acc.address,
        ip          : ip.to_string(),
        secret_key  : sk,
        trusted     : "".to_string(),
    };

    save_profile(&p, conn);

    //Creating a new profile also activates it.

    match get_active(conn){
        Err(_)  => activate(n, conn),
        Ok(_)   => switch_active(n, conn),
    }
    return p;
}

pub fn trusted_nodes(conn: &Connection) -> Vec<String>{
    let p = get_active(conn).unwrap();
    let raw_trusted: String = p.trusted;
    helper::slice_to_vec(&raw_trusted)
}

//Activate profile of a given name
pub fn activate(name: &str, conn: &Connection){
    println!("\nActivating profile...");

    //Check if there is a profile activated
    match get_active(conn){
        Err(_) => {
            let mut p = get_profile(name, conn);
            p.active = true;
            save_profile(&p, conn);
            println!("Profile activated.");
        },
        Ok(p) => {
            if p.name != name{
                println!("Profile {:?} is currently active. Deactivating and activating {:?}", p.name, name);
                switch_active(name, conn);
            } else {
                println!("Profile {:?} is already active.", p.name);
                let mut p = get_profile(name, conn);
                p.active = true;

                save_profile(&p, conn);
                println!("Profile activated.");
            }
        },
    }

}

//Deactive current profile
pub fn deactivate(conn: &Connection){
    let mut p = get_active(conn).unwrap();
    p.active = false;
    save_profile(&p, conn);
}
