extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;
use super::*;
use postgres::{Connection, SslMode, error};

// Connect to database.
// Output   Connection      Database connection.
pub fn connect_db() -> Connection{
    let conn = Connection::connect("postgresql://postgres:api@localhost", &SslMode::None).unwrap();
    return conn;
}

// Close database connection.
// Input    Connection      Database connection to be consumed.
pub fn close_db(conn: Connection){
    let _ = Connection::finish(conn);
}

// Check tables returns missing tables
// Input    Connection      Database connection.
// Output   Vec<String>     Name of missing tables.
pub fn check_tables(conn: &Connection) -> Vec<String>{
    let maybe_stmt = conn.prepare("select * from pg_tables where schemaname='public'");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    //Variable represents missing tables
    let mut tables: Vec<String> = vec!["account".to_string(), "profile".to_string(),
                                        "log".to_string(), "state".to_string()];
    let rows = stmt.query(&[]).unwrap();
    for row in rows {
        let table_name: String = row.get(1);
        let remove = tables.iter().position(|n| n == &table_name);
        tables.remove(remove.unwrap());
    }
    return tables;
}

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
