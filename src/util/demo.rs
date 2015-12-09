extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use data::log;

// Retrieve preloaded logs for demonstration
// Input    code    Log identifier (a, b, c)
// Input    fuel    Fuel of selected log
// Output   log     Preloaded log
pub fn get_demo_log(code: &str, fuel: i64)-> log::log{
    let mut l = log::log{
        hash    :   "".to_string(),
        state   :   "".to_string(),
        nonce   :   0 as i64,
        origin  :   "".to_string(),
        target  :   "".to_string(),
        fuel    :   0 as i64,
        code    :   "".to_string(),
        sig     :   Vec::new(),
    };
    let code_a: String = "LOAD 1,LOAD 2,POP 2,ADD 2,PUSH 1,LOAD 2,LOAD 4,LOAD 6,POP 3,MUL 3,PUSH 1,STOP".to_string();
    // let code_b: String = "LOAD 1,POP 1,PC 1,PC 3,POP 1,PUSH 1,STOP".to_string();
    let code_c: String = "LOAD 2,LOAD 4,LOAD 6,POP 3,MUL 3,PUSH 1,STOP".to_string();
    match &code[..]{
        "a" => {l.code = code_a;},
        // "b" => {l.code = code_b;},
        "c" => {l.code = code_c;},
        _ => {println!{"You entered an invalid character. Please try again."}},
    }
    l.fuel = fuel;
    return l;
}
