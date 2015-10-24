extern crate turbine;
extern crate rand;

use turbine::data::account;
use self::rand::{OsRng, Rng};


#[test]
fn test_create_new_account(){
    let mut schain_add =  rand::thread_rng().gen_iter::<u8>().take(20).collect::<Vec<u8>>();
    // for n in &schain_add{
    //     print!("{}", n);
    // }
    let string_add = std::str::from_utf8(&schain_add);
    println!("Randomly generated address: {:?}", string_add);
    account::create_new_account(&schain_add);
}
