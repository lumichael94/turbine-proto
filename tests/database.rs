extern crate turbine;
use turbine::data::database;


#[test]
fn test_connect_db(){
    database::connect_db();
}

#[test]
fn test_close_db(){
    let conn = database::connect_db();
    database::close_db(conn);
}
