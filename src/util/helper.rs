extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;
use std::io::{self, Write, Read, BufRead};

// Convert slice to string vector
// Input    text            Slice to convert (&[u8])
// Output   Vec<String>     String vector
pub fn slice_to_vec(text: &str) -> Vec<String>{
    let s: String = text.to_string();
    let split = s.split(",");
    let coll = split.collect::<Vec<&str>>();
    return vec_slice_to_string(&coll);
}

// Convert vector of str to string vector
// Input    v               Vector of &str
// Output   Vec<String>     String vector
pub fn vec_slice_to_string(v: &Vec<&str>) -> Vec<String>{
    let mut vec: Vec<String> = Vec::new();
    for x in v {
        vec.push(x.to_string());
    }
    return vec;
}

//====================================================================
//USER INPUT FUNCTIONS
//====================================================================

// Reads and returns user response.
// Output   String      User input.
pub fn read_in() -> String{
    print!("=>> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut response = String::new();
    let _ = stdin.read_line(&mut response);

    //Remove "\n" from response
    let valid = response.len() - 1;
    response.truncate(valid);
    return response;
}

// Reads response to yes or no prompt.
// Output   Boolean      Yes/No
pub fn read_yn() -> bool{
    let response: String = read_in();
    let yn = match &response[..] {
        "y"|"Y"|"yes"|"Yes"|"YES"   => true,
        "n"|"N"|"no"|"No"|"NO"      => false,
        _                           => {
            println!("Invalid response. Try again.");
            return read_yn();
        },
    };
    return yn;
}
