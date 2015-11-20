extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;

use vm::opCodes::{opCode, opCode_param, map_to_fuel, map_to_fn};
use vm::opCodes::opCode::*;
use vm::decoder::{decode, map_to_string};
use data::log::log;
use data::account::account;
use util::{krypto, helper};

pub struct env_state {
    pub origin      : String,
    pub target      : String,
    pub stack       : Vec<i32>,
    pub pc          : i64,
    pub memory      : Vec<i32>,
    pub code_hash   : String,
    pub fuel        : i64,
    pub code        : Vec<String>,
}

//TODO: Implement
// pub fn init_from_slice(){
//
// }

pub fn execute_code(mut state: &mut env_state,
    instr_set: &(Vec<opCode>, Vec<Vec<String>>, Vec<i64>)) -> log{
    loop {
        let ref instr: opCode       = (instr_set.0)[state.pc as usize];
        let ref param: Vec<String>    = (instr_set.1)[state.pc as usize];
        let ref fuel_cost: i64      = (instr_set.2)[state.pc as usize];

        // println!("\n\nExecute opCode: {}", map_to_string(&instr));
        // println!("With params of: {:?}", param);
        // println!("And a fuel cost of: {:?}", fuel_cost);

        execute_instr(&instr, &param, state);

        // println!("This is the stack: {:?}", state.stack);
        // println!("This is the memory: {:?}\n\n", state.memory);
        // println!("This is the counter: {:?}", state.pc);

        if state.pc < 0 {
            break;
        }
        state.pc+=1;
    }
    log_from_env(state)
}

pub fn execute_instr(instr: &opCode, param: &Vec<String>, mut state: &mut env_state){

    match instr {
        &ADD     => {
            state.code_hash = krypto::string_hash(&state.code_hash, "ADD").to_string();
            // state.fuel -= ;
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            // map_to_fn(opCode_param::ADD(&mut state.stack, &mut state.memory, n));
            map_to_fn(opCode_param::ADD(state, n));
        },
        &MUL     => {
            state.code_hash = krypto::string_hash(&state.code_hash, "MUL").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::MUL(state, n));
        },
        &DIV     => {
            state.code_hash = krypto::string_hash(&state.code_hash, "DIV").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::DIV(state, n));
        },
        &MOD     =>{
            state.code_hash = krypto::string_hash(&state.code_hash, "MOD").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::MOD(state, n));
        },
        &SUB     => {
            state.code_hash = krypto::string_hash(&state.code_hash, "SUB").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::SUB(state, n));
        },
        &POP     => {
            state.code_hash = krypto::string_hash(&state.code_hash, "POP").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::POP(state, n));
        },
        &LOAD    =>{
            state.code_hash = krypto::string_hash(&state.code_hash, "LOAD").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::LOAD(state, n));
        },
        &PUSH    =>{
            state.code_hash = krypto::string_hash(&state.code_hash, "PUSH").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::PUSH(state, n));
        },
        &STOP    =>{
            state.code_hash = krypto::string_hash(&state.code_hash, "STOP").to_string();
            map_to_fn(opCode_param::STOP(state));
        },
        &ERROR   => {
            state.code_hash = krypto::string_hash(&state.code_hash, "ERROR").to_string();
            map_to_fn(opCode_param::ERROR(state));
        },
    }
}

pub fn new_proof(env: &mut env_state) -> String{
    let a = krypto::string_hash(&env.code_hash, &env.origin);
    return krypto::string_hash(&a, &(env.fuel).to_string());
}

pub fn env_from_account(acc: &account) -> env_state{
    env_state{  origin:     (*acc.address).to_string(),
                target:     "".to_string(),
                stack:      Vec::new(),
                pc:         0,
                memory:     Vec::new(),
                code_hash:  "".to_string(),
                fuel:       acc.fuel,
                code:       helper::format_code(&acc.address),}
}

pub fn log_from_env(mut env: &mut env_state) -> log{

    log{    hash:       "hash".to_string(),
            block:      "block".to_string(),
            nonce:      872635,
            origin:     "origin".to_string(),
            target:     "target".to_string(),
            fuel:       env.fuel,
            sig:        "signature".to_string(),
            proof:      new_proof(env),
        }
}

//Both tests works. Uncomment and edit if necessary.

#[cfg(test)]
mod test {
  use super::*;
  use vm::opCodes::{opCode, opCode_param, map_to_fn};
  use vm::opCodes::opCode::*;
  use vm::decoder::decode;
  use data::log;
  use data::account;
  use util::helper::format_code;


  // #[test]
  // fn test_env1() {
  //   println!("\n\n\n\nenv test 1"); //Works!
  //
  //   let mut env = env_state{stack: Vec::new(), pc: 0, memory: Vec::new(), code_hash: "".to_string()};
  //   let x: i32 = 7;
  //   let y: i32 = 2;
  //   map_to_fn(opCode_param::LOAD(&mut env.stack, x));
  //   map_to_fn(opCode_param::LOAD(&mut env.stack, y));
  //   map_to_fn(opCode_param::POP(&mut env.stack, &mut env.memory, 2));
  //   println!("\n\nLoaded variables");
  //   println!("This is the stack: {:?}", env.stack);
  //   println!("This is the memory: {:?}\n\n", env.memory);
  //
  //   map_to_fn(opCode_param::MOD(&mut env.stack, &mut env.memory, 2));
  //
  //   println!("This is the stack: {:?}", env.stack);
  //   println!("This is the memory: {:?}", env.memory);
  //
  //   map_to_fn(opCode_param::PUSH(&mut env.stack, &mut env.memory, 1));
  //
  //   println!("\n\nOperation finished.");
  //   println!("This is the stack: {:?}", env.stack);
  //   println!("This is the memory: {:?}", env.memory);
  //
  //   map_to_fn(opCode_param::STOP(&mut env.pc));
  //   println!("This is the program counter: {:?}\n\n", env.pc);
  // }

  #[test]
  fn test_execute_code() {
    println!("\n\n\n\n\n\nvm test 2");
    let code_text: String = "LOAD 1,LOAD 2,POP 2,ADD 2,PUSH 1,STOP".to_string();
    let acc = account::account {    address:    "my address".to_string(),
                                    ip:         "192.168.1.1".to_string(),
                                    trusted:    false,
                                    log_nonce:  2,
                                    fuel:       500,
                                    code:       code_text,};

    let mut env = env_from_account(&acc);
    let code: Vec<String> = format_code(&acc.code);
    let instr_set: (Vec<opCode>, Vec<Vec<String>>, Vec<i64>) = decode(&code);
    let l: log::log = execute_code(&mut env, &instr_set);

    let hash: String = env.code_hash;
    println!("\n\n\nCode hash: {:?}", hash);
    println!("Stack: {:?}", env.stack);
    println!("Memory: {:?}", env.memory);
    println!("PC: {:?}", env.pc);
    println!("Origin: {:?}", l.origin);
    println!("Fuel: {:?}", l.fuel);
    println!("Proof: {:?}\n\n\n", l.proof);

  }
}
