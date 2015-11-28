extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;
use super::*;
use postgres::{Connection, SslMode};

//Connect to database.
pub fn connect_db() -> Connection{
    let conn = Connection::connect("postgresql://postgres:api@localhost", &SslMode::None).unwrap();
    return conn;
}

//Close database connection.
pub fn close_db(conn: Connection){
    let _ = Connection::finish(conn);
}

//Check tables returns missing tables
pub fn check_tables(conn: &Connection) -> Vec<String>{
    //TODO: This statement may grab unrelated tables from the user's postgresql
    let maybe_stmt = conn.prepare("select * from pg_tables where schemaname='public'");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    //Variable represents missing tables
    let mut tables: Vec<String> = vec!["account".to_string(), "local".to_string(),
                                        "log".to_string(), "state".to_string()];
    let rows = stmt.query(&[]).unwrap();
    for row in rows {
        let table_name: String = row.get(1);
        let remove = tables.iter().position(|n| n == &table_name);
        //TODO: Check if this works if "remove.unwrap()" returns "None"
        //If fails, then check https://doc.rust-lang.org/std/option/enum.Option.html
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
