extern crate turbine;
extern crate rand;
extern crate postgres;

use turbine::data::account;
use turbine::data::database;
use postgres::{Connection, SslMode};

#[test]
fn test_store_account(){
    let conn = database::connect_db();
    account::create_account_table(&conn);

    let ip: &str = "192.168.1.1";
    let acc = account::account {    address:    "address".to_string(),
                                    ip:         "192.168.1.1".to_string(),
                                    trusted:    false,
                                    log_nonce:  0 as i64,
                                    fuel:        0 as i64,
                                    code:       "".to_string(),};

    account::save_account(&acc, &conn);

    let a = account::get_account(&acc.address, &conn);

    account::drop_account_table(&conn);
    database::close_db(conn);
}
