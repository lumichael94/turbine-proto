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
    log::setup_log_table(&conn);

    let l = log::new_log("block", "id", "origin_address", "target", "fuel");
    log::store_log(&l, &conn);

    let retrieved_log = log::get_log(&l.id, &conn);

    log::drop_log_table(&conn);
    database::close_db(conn);
}
