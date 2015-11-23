extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use std::net;
use network::{server, proto};
use data::{account, state, database,log};
use vm::env;
use util::{helper, krypto};
use self::postgres::{Connection, SslMode};
use std::thread;

pub fn init() -> Connection{
    //Database
    let conn: Connection = database::connect_db();
    account::create_account_table(&conn);
    state::create_state_table(&conn);
    log::create_log_table(&conn);

    let my_acc = account::new_account();
    account::save_account(&my_acc, &conn);

    //TCP Server
    let server_thread = thread::spawn(move||{
                            server::listen("127.0.0.1:8888");
                        });


    let client_thread = thread::spawn(move ||{
                            proto::connect_to_peers();
                        });
                        
    let _ = server_thread.join();
    let _ = client_thread.join();

    return conn;
}

// pub fn main() {
//     println!("Hello World!");
//     let conn: Connection = init();
//     end(conn);
// }

pub fn end(conn: Connection) {
    // account::drop_account_table(&conn);
    // state::drop_state_table(&conn);
    // log::drop_log_table(&conn);
    database::close_db(conn);
}

pub fn drop_everything(){
    let conn = database::connect_db();
    account::drop_account_table(&conn);
    state::drop_state_table(&conn);
    log::drop_log_table(&conn);
    database::close_db(conn);
}
