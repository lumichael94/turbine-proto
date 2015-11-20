extern crate rand;
extern crate crypto;
extern crate rustc_serialize;

use vm::env::env_state;

pub enum opCode {
    ADD, MUL, DIV, MOD, SUB,
    POP, LOAD, PUSH, STOP, ERROR,
    // STORE, STORE8, JUMP, FUEL, JUMPDEST, PC
    // ADDRESS, GUAGE, ORIGIN,
    // DUP, SWAP,CREATE, CALL, RETURN,
}

pub enum opCode_param<'a> {

    //Stop and Arithmetic Operations
    // STOP,
    // ADD(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    ADD(&'a mut env_state, i32),
    SUB(&'a mut env_state, i32),
    POP(&'a mut env_state, i32),
    PUSH(&'a mut env_state, i32),
    MUL(&'a mut env_state, i32),
    DIV(&'a mut env_state, i32),
    MOD(&'a mut env_state, i32),
    LOAD(&'a mut env_state, i32),
    STOP(&'a mut env_state),
    ERROR(&'a mut env_state),

    //Comparison and Bitwise Logic Operations
    // LESS, GREAT, EQUAL, ISZERO, AND,
    // OR, XOR, NOT, BYTE,

    //Environment Information
    // ADDRESS, GUAGE, ORIGIN,
    // CODESIZE, CODECOPY,

    //Block Information
    // BLOCKHASH, TIMESTAMP, BNONCE, BGUAGE,

    //Stack, Memory, Storage, and Flow Operations
    // POP, LOAD, STORE, STORE8, JUMP,
    // JUMPIF, PC, MSIZE, GAS, JUMPDEST,

    //Using an 64 word size
    //Push, Duplication, Exchange Operations
    // PUSH1, PUSH2, PUSH3, PUSH4, PUSH5, PUSH6, PUSH7, PUSH8,
    // DUP1, DUP2, DUP3, DUP4, DUP5, DUP6, DUP7, DUP8,
    // SWAP1, SWAP2, SWAP3, SWAP4, SWAP5, SWAP6, SWAP7, SWAP8,

    //System Operations
    // CREATE, CALL, CALLCODE, RETURN, SUICIDE,
}

pub fn map_to_fn(code: opCode_param) {
    match code {

        // opCode_param::ADD(stack, memory, n)      => add(stack, memory, n),
        opCode_param::ADD(env, n)                => add(env, n),
        opCode_param::SUB(env, n)      => sub(env, n),
        opCode_param::MUL(env, n)      => mul(env, n),
        opCode_param::DIV(env, n)      => div(env, n),
        opCode_param::MOD(env, n)      => modulo(env, n),

        //TODO: Change this when you can
        opCode_param::LOAD(env, word)          => load(env, word),
        opCode_param::PUSH(env, n)     => push(env, n),
        opCode_param::POP(env, n)      => pop(env, n),

        opCode_param::STOP(env)       => stop(env),
        opCode_param::ERROR(env)      => error(env),

    };
}

// //Number of params
// pub fn get_param_len(code: &str) -> i32{
//     match code {
//         "ADD"     => return 1,
//         "MUL"     => return 1,
//         "DIV"     => return 1,
//         "MOD"     => return 1,
//         "SUB"     => return 1,
//         "POP"     => return 0,
//         "LOAD"    => return 1,
//         "PUSH"    => return 1,
//         "STOP"    => return 0,
//         "ERROR"   => return 0,
//         _       => return 0,
//     };
// }


// fn add(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
//     let mut ans = memory.pop().unwrap();
//     for i in 0..(n-1){
//         ans += memory.pop().unwrap();
//     }
//     memory.push(ans);
//     println!("ADD: {}", memory.last().unwrap());
// }

fn add(mut env: &mut env_state, n: i32){
    let mut ans = env.memory.pop().unwrap();
    for i in 0..(n-1){
        ans += env.memory.pop().unwrap();
    }
    env.memory.push(ans);
    // println!("ADD: {}", env.memory.last().unwrap());
}

fn sub(mut env: &mut env_state, n: i32){
    let mut ans = env.memory.pop().unwrap();
    for i in 0..(n-1){
        ans -= env.memory.pop().unwrap();
    }
    env.memory.push(ans);
    // println!("SUB: {}", env.memory.last().unwrap());
}

fn mul(mut env: &mut env_state, n: i32){
    let mut ans = env.memory.pop().unwrap();
    for i in 0..(n-1){
        ans *= env.memory.pop().unwrap();
    }
    env.memory.push(ans);
    // println!("MUL: {}", env.memory.last().unwrap());
}

fn div(mut env: &mut env_state, n: i32){
    let mut ans = env.memory.pop().unwrap();
    for i in 0..(n-1){
        ans /= env.memory.pop().unwrap();
    }
    env.memory.push(ans);
    // println!("DIV: {}", env.memory.last().unwrap());
}

fn modulo(mut env: &mut env_state, n: i32){
    let mut ans = env.memory.pop().unwrap();
    for i in 0..(n-1){
        ans %= env.memory.pop().unwrap();
    }
    env.memory.push(ans);
    // println!("MOD: {}", env.memory.last().unwrap());
}

fn pop(mut env: &mut env_state, n: i32){
    for i in 0..n{
        env.memory.push(env.stack.pop().unwrap());
    }
    // println!("POP")
}

fn push(mut env: &mut env_state, n: i32){
    for i in 0..n{
        env.stack.push(env.memory.pop().unwrap());
    }
    // println!("PUSH")
}

fn load(mut env: &mut env_state, word: i32){
    env.stack.push(word);
    // println!("LOAD")
}

fn stop(mut env: &mut env_state){
    env.pc = -1;
    // println!("STOP");
}

fn error(mut env: &mut env_state){
    env.pc = -2;
    // println!("ERROR");
}

pub fn map_to_fuel(code: opCode) -> i64{
    match code{
        opCode::ADD => return 1,
        opCode::SUB => return 1,
        opCode::MUL => return 5,
        opCode::DIV => return 5,
        opCode::MOD => return 13,
        opCode::POP => return 2,
        opCode::LOAD => return 7,
        // opPrice::STORE => return 11,
        // opPrice::STORE8 => return 31,
        // opPrice::JUMP => return 19,
        // opPrice::PC => return 23,
        // opPrice::FUEL => return 13,
        // opPrice::JUMPDEST => return 19,
        // opPrice::ADDRESS => return 43,
        // opPrice::GUAGE => return 29,
        // opPrice::ORIGIN => return 43,
        opCode::PUSH => return 1,
        // opPrice::DUP => return 1,
        // opPrice::SWAP => return 1,
        // opPrice::CREATE => return 61,
        // opPrice::CALL => return 59,
        // opPrice::RETURN => return 3,
        opCode::STOP => return 0,
        opCode::ERROR => return 109,
    }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_opCodes() {
    println!("opCode test");
    // let x = map_to_fuel(opPrice::ADD);
    // println!("ADD returns fuel of 1: {}", x);
  }
}
