extern crate turbine;
extern crate rand;
extern crate postgres;

use turbine::data::account;
use turbine::data::database;
use self::rand::{OsRng, Rng};
use postgres::{Connection, SslMode};

#[test]
fn test_store_account(){
    let conn = database::connect_db();
    account::create_account_table(&conn);

    let add = account::gen_account_address();
    let ip: &str = "192.168.1.1";
    let acc = account::create_new_account(&add, ip);

    account::save_account(&acc, &conn);

    let a = account::get_account(&acc.address, &conn);

    account::drop_account_table(&conn);
    database::close_db(conn);
}
