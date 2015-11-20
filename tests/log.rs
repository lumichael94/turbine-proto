extern crate turbine;
extern crate rand;
extern crate postgres;

use turbine::data::log;
use turbine::data::database;
use self::rand::{OsRng, Rng};
use postgres::{Connection, SslMode};

#[test]
fn test_store_log(){
    let conn = database::connect_db();
    log::create_log_table(&conn);

    // let l = log::new_log("block", "id", "origin_address", "sig");
    let l = log::log{   hash:       "hash".to_string(),
                        block:      "block".to_string(),
                        nonce:      872635,
                        origin:     "origin".to_string(),
                        fuel:       567890,
                        sig:        "signature".to_string(),
                        proof:      "proof".to_string(),};

    log::save_log(&l, &conn);

    let retrieved_log: log::log = log::get_log(&l.hash, &conn);
    println!("The retrieved log hash: {:?}", retrieved_log.hash);

    log::drop_log_table(&conn);
    database::close_db(conn);
}
