extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;
use postgres::{Connection, SslMode};

//  Connect to database.
pub fn connect_db() -> Connection{
    let conn = Connection::connect("postgresql://postgres:api@localhost", &SslMode::None).unwrap();
    return conn;
}

//  Close database connection.
pub fn close_db(conn: Connection){
    let _ = Connection::finish(conn);
}
