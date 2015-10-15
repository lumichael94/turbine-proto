extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;
use postgres::{Connection, SslMode};

//  Connect to database.
//  TODO: Return error message if failure to connect.
pub fn connect_db() -> Connection{
    let conn = Connection::connect("postgresql://postgres:api@localhost", &SslMode::None).unwrap();
    return conn;
}

//  Close database connection.
//  TODO: Return error message if failure to close.
//  TODO: Ignoring return type
pub fn close_db(connection: Connection){
    let _ = Connection::finish(connection);
}
