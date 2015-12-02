extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate secp256k1;

use vm::opCodes::{opCode, opCode_param, map_to_fuel, map_to_fn};
use vm::opCodes::opCode::*;
use vm::decoder::{decode, map_to_string};
use postgres::{Connection, SslMode};
use data::log;
use data::state::state;
use data::{account, database, profile};
use util::{krypto, helper};
use self::secp256k1::*;
use self::secp256k1::key::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct env {
    pub origin      : account::account,
    pub targets     : Vec<account::account>,
    pub env_log     : log::log,
    pub hash        : String,       // Also known as proof
}

// TODO: Change output env into Option<env>
// Initializes an env from a log and its traits
pub fn env_from_log(curr_accs: Arc<RwLock<HashMap<String, account::account>>>, l: log::log) -> env{
    // TODO: Check if origin account is retrieved correctly
    let accs = curr_accs.read().unwrap();
    let env_origin = accs.get(&l.origin).unwrap().clone();
    return env{
        origin:     env_origin,
        targets:    Vec::new(),
        env_log:    l,
        hash:       "".to_string(),
    }
}

pub fn propose_state(curr_accs: Arc<RwLock<HashMap<String, account::account>>>,
    logs: Arc<RwLock<HashMap<String, log::log>>>) -> state{

    let log_hmap: HashMap<String, log::log> = logs.read().unwrap().clone();

    // Executing Logs
    for (l_hash, l) in log_hmap{
        let code: Vec<String> = helper::slice_to_vec(&l.code);
        let instr_set: (Vec<opCode>, Vec<Vec<String>>, Vec<i64>) = decode(&code);
        let arc = curr_accs.clone();
        let mut e: env = env_from_log(arc, l);
        let env_hash: String = execute_env(true, &mut e, &instr_set);
        if l_hash != env_hash{
            // TODO: Invalid Log, Request from trusted nodes.
            println!("There is an invalid log.");
        }
    }

    return state {
        nonce:      0 as i64,
        hash:       "".to_string(),
        prev_state: "".to_string(),
        acc_hash:   "".to_string(),
        l_hash:     "".to_string(),
        fuel_exp:   0 as i64,
    };
}

pub fn execute_env(sign: bool, e: &mut env,
    instr_set: &(Vec<opCode>, Vec<Vec<String>>, Vec<i64>)) -> String{
    loop {
        // TODO: Not sure if assigning a variable as e.origin.pc will destroy reference
        let ref instr: opCode       = (instr_set.0)[e.origin.pc as usize];
        let ref param: Vec<String>  = (instr_set.1)[e.origin.pc as usize];
        // let ref fuel_cost: i64      = (instr_set.2)[e.origin.pc as usize];

        // println!("\n\nExecute opCode: {}", map_to_string(&instr));
        // println!("With params of: {:?}", param);
        // println!("And a fuel cost of: {:?}", fuel_cost);

        execute_instr(&instr, &param, e);

        // println!("This is the stack: {:?}", e.stack);
        // println!("This is the memory: {:?}\n\n", e.memory);
        // println!("This is the counter: {:?}", e.pc);

        if e.origin.pc < 0 {
            break;
        }
        e.origin.pc+=1;
    }
    e.hash.to_string()
}

pub fn execute_instr(instr: &opCode, param: &Vec<String>, mut e: &mut env){

    match instr {
        &ADD     => {
            e.hash = krypto::string_hash(&e.hash, "ADD").to_string();
            // e.fuel -= ;
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            // map_to_fn(opCode_param::ADD(&mut e.stack, &mut e.memory, n));
            map_to_fn(opCode_param::ADD(e, n));
        },
        &MUL     => {
            e.hash = krypto::string_hash(&e.hash, "MUL").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::MUL(e, n));
        },
        &DIV     => {
            e.hash = krypto::string_hash(&e.hash, "DIV").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::DIV(e, n));
        },
        &MOD     =>{
            e.hash = krypto::string_hash(&e.hash, "MOD").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::MOD(e, n));
        },
        &SUB     => {
            e.hash = krypto::string_hash(&e.hash, "SUB").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::SUB(e, n));
        },
        &POP     => {
            e.hash = krypto::string_hash(&e.hash, "POP").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::POP(e, n));
        },

        &SEND     => {
            e.hash = krypto::string_hash(&e.hash, "SEND").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::SEND(e, n));
        },

        &JUMP     => {
            e.hash = krypto::string_hash(&e.hash, "JUMP").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::JUMP(e, n));
        },

        &LOAD    =>{
            e.hash = krypto::string_hash(&e.hash, "LOAD").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::LOAD(e, n));
        },
        &PUSH    =>{
            e.hash = krypto::string_hash(&e.hash, "PUSH").to_string();
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::PUSH(e, n));
        },
        &STOP    =>{
            e.hash = krypto::string_hash(&e.hash, "STOP").to_string();
            map_to_fn(opCode_param::STOP(e));
        },
        &ERROR   => {
            e.hash = krypto::string_hash(&e.hash, "ERROR").to_string();
            map_to_fn(opCode_param::ERROR(e));
        },
    }
}

pub fn new_proof(env: &mut env) -> String{
    let a = krypto::string_hash(&env.hash, &env.origin.address);
    return krypto::string_hash(&a, &(env.env_log.fuel).to_string());
}

//The sign parameter asks whether or not the local account should sign.
pub fn log_from_env(mut env: &mut env, sign: bool) -> log::log{
    let l_state:    String  = (*env.origin.state).to_string();
    let l_nonce:    i64     = env.origin.log_nonce;
    let l_origin:   String  = (*env.env_log.hash).to_string();
    let l_target:   String  = (*env.env_log.target).to_string();
    let l_fuel:     i64     = env.env_log.fuel;
    let l_code:     String  = (*env.env_log.code).to_string();
    let l_proof:    String  = new_proof(env);

    let mut a: String = krypto::string_int_hash(&l_state, &l_nonce);
    a = krypto::string_hash(&a, &l_origin);
    a = krypto::string_hash(&a, &l_target);
    a = krypto::string_int_hash(&a, &l_fuel);
    a = krypto::string_hash(&a, &l_proof);

    let l_sig: Vec<u8>; // = (*env.env_log.sig).to_string();

    if sign == true{
        let conn = database::connect_db();
        let profile = profile::get_active(&conn).unwrap();

        //TODO: Where to increment log nonce?
        let sk: SecretKey = krypto::decode_sk(&profile.secret_key);
        let signed: Signature = krypto::sign_message(l_proof.as_bytes(), &sk).unwrap();
        let engine = Secp256k1::new();
        l_sig = signed.serialize_der(&engine);
        database::close_db(conn);
    } else {
        l_sig = Vec::new();
    }

    log::log{    hash:       a.to_string(),
            state:      l_state,
            nonce:      l_nonce,
            origin:     l_origin,
            target:     l_target,
            fuel:       l_fuel,
            code:       l_code,
            sig:        l_sig,
        }
}

//Both tests work. Uncomment and edit if necessary.

// #[cfg(test)]
// mod test {
//   use super::*;
//   use vm::opCodes::{opCode, opCode_param, map_to_fn};
//   use vm::opCodes::opCode::*;
//   use vm::decoder::decode;
//   use data::log;
//   use data::account;
//   use util::helper::format_code;


  // #[test]
  // fn test_env1() {
  //   println!("\n\n\n\nenv test 1"); //Works!
  //
  //   let mut env = env{stack: Vec::new(), pc: 0, memory: Vec::new(), code_hash: "".to_string()};
  //   let x: i32 = 7;
  //   let y: i32 = 2;
  //   map_to_fn(opCode_param::LOAD(&mut env.origin.stack, x));
  //   map_to_fn(opCode_param::LOAD(&mut env.origin.stack, y));
  //   map_to_fn(opCode_param::POP(&mut env.origin.stack, &mut env.memory, 2));
  //   println!("\n\nLoaded variables");
  //   println!("This is the stack: {:?}", env.origin.stack);
  //   println!("This is the memory: {:?}\n\n", env.memory);
  //
  //   map_to_fn(opCode_param::MOD(&mut env.origin.stack, &mut env.memory, 2));
  //
  //   println!("This is the stack: {:?}", env.origin.stack);
  //   println!("This is the memory: {:?}", env.memory);
  //
  //   map_to_fn(opCode_param::PUSH(&mut env.origin.stack, &mut env.memory, 1));
  //
  //   println!("\n\nOperation finished.");
  //   println!("This is the stack: {:?}", env.origin.stack);
  //   println!("This is the memory: {:?}", env.memory);
  //
  //   map_to_fn(opCode_param::STOP(&mut env.pc));
  //   println!("This is the program counter: {:?}\n\n", env.pc);
  // }
//
//   #[test]
//   fn test_execute_code() {
//     println!("\n\n\n\n\n\nvm test 2");
//     let code_text: String = "LOAD 1,LOAD 2,POP 2,ADD 2,PUSH 1,STOP".to_string();
//
//     let acc = account::account {    address:    "my address".to_string(),
//                                         ip:         "192.168.1.1".to_string(),
//                                         trusted:    false,
//                                         log_nonce:  2,
//                                         fuel:       500,
//                                         code:       code_text,
//                                         state:      "state address".to_string(),
//                                         public_key: "public_key".to_string()
//                                 };
//
//     let l = log::log{   hash:       "hash".to_string(),
//                         state:      "state".to_string(),
//                         nonce:      872635,
//                         origin:     "origin".to_string(),
//                         target:     "target".to_string(),
//                         fuel:       567890,
//                         sig:        "signature".to_string(),
//                         proof:      "".to_string(),};
//
//     let mut env = new_env(acc, l);
//     let code: Vec<String> = format_code(&env.env_acc.code);
//     let instr_set: (Vec<opCode>, Vec<Vec<String>>, Vec<i64>) = decode(&code);
//     let l: log::log = execute_code(&mut env, &instr_set);
//
//     let hash: String = env.code_hash;
//     println!("\n\n\nCode hash: {:?}", hash);
//     println!("Stack: {:?}", env.origin.stack);
//     println!("Memory: {:?}", env.memory);
//     println!("PC: {:?}", env.pc);
//     println!("Origin: {:?}", l.origin);
//     println!("Fuel: {:?}", l.fuel);
//     println!("Proof: {:?}\n\n\n", l.proof);
//
//   }
