extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;
use super::*;
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
// 
// #[cfg(test)]
// mod test {
//     extern crate postgres;
//
//     use super::*;
//     use postgres::{Connection, SslMode};
//
//     #[test]
//     fn test_connect_db(){
//         connect_db();
//     }
//
//     #[test]
//     fn test_close_db(){
//         let conn = connect_db();
//         close_db(conn);
//     }
// }
