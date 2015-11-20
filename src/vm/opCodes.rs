extern crate rand;
extern crate crypto;
extern crate rustc_serialize;


pub enum opCode<'a> {

    //Stop and Arithmetic Operations
    // STOP,
    ADD(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    SUB(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    POP(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    PUSH(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    MUL(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    DIV(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    MOD(&'a mut Vec<i32>, &'a mut Vec<i32>, i32),
    LOAD(&'a mut Vec<i32>, i32),
    STOP(&'a mut i64),

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


pub enum opPrice {
    ADD, MUL, DIV, MOD, SUB,
    POP, LOAD, STORE, STORE8, JUMP, PC, FUEL, JUMPDEST,
    ADDRESS, GUAGE, ORIGIN,
    PUSH, DUP, SWAP,
    CREATE, CALL, RETURN, STOP,
}

pub fn map_to_fn(code: opCode) {
    match code {
        opCode::STOP(pc)                        => stop(pc),
        opCode::ADD(stack, memory, n)           => add(stack, memory, n),
        opCode::SUB(stack, memory, n)           => sub(stack, memory, n),
        opCode::MUL(stack, memory, n)           => mul(stack, memory, n),
        opCode::DIV(stack, memory, n)           => div(stack, memory, n),
        opCode::MOD(stack, memory, n)           => modulo(stack, memory, n),

        //TODO: Change this when you can
        opCode::LOAD(stack, word)          => load(stack, word),
        opCode::PUSH(stack, memory, n)     => push(stack, memory, n),
        opCode::POP(stack, memory, n)      => pop(stack, memory, n),
    };
}

fn add(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
    let mut ans = memory.pop().unwrap();
    for i in 0..(n-1){
        ans += memory.pop().unwrap();
    }
    memory.push(ans);
    println!("ADD: {}", memory.last().unwrap());
}

fn sub(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
    let mut ans = memory.pop().unwrap();
    for i in 0..(n-1){
        ans -= memory.pop().unwrap();
    }
    memory.push(ans);
    println!("SUB: {}", memory.last().unwrap());
}

fn mul(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
    let mut ans = memory.pop().unwrap();
    for i in 0..(n-1){
        ans *= memory.pop().unwrap();
    }
    memory.push(ans);
    println!("MUL: {}", memory.last().unwrap());
}

fn div(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
    let mut ans = memory.pop().unwrap();
    for i in 0..(n-1){
        ans /= memory.pop().unwrap();
    }
    memory.push(ans);
    println!("DIV: {}", memory.last().unwrap());
}

fn modulo(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
    let mut ans = memory.pop().unwrap();
    for i in 0..(n-1){
        ans %= memory.pop().unwrap();
    }
    memory.push(ans);
    println!("MOD: {}", memory.last().unwrap());
}

fn pop(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
    for i in 0..n{
        memory.push(stack.pop().unwrap());
    }
    println!("POP")
}

fn push(mut stack: &mut Vec<i32>, memory: &mut Vec<i32>, n: i32){
    for i in 0..n{
        stack.push(memory.pop().unwrap());
    }
    println!("PUSH")
}

fn load(mut stack: &mut Vec<i32>, word: i32){
    stack.push(word);
    println!("LOAD")
}

fn stop(mut pc: &mut i64){
    *pc = -1;
    println!("STOP");
}

pub fn map_to_fuel(code: opPrice) -> i64{
    match code{
        opPrice::ADD => return 1,
        opPrice::SUB => return 1,
        opPrice::MUL => return 5,
        opPrice::DIV => return 5,
        opPrice::MOD => return 13,
        opPrice::POP => return 2,
        opPrice::LOAD => return 7,
        opPrice::STORE => return 11,
        opPrice::STORE8 => return 31,
        opPrice::JUMP => return 19,
        opPrice::PC => return 23,
        opPrice::FUEL => return 13,
        opPrice::JUMPDEST => return 19,
        opPrice::ADDRESS => return 43,
        opPrice::GUAGE => return 29,
        opPrice::ORIGIN => return 43,
        opPrice::PUSH => return 1,
        opPrice::DUP => return 1,
        opPrice::SWAP => return 1,
        opPrice::CREATE => return 61,
        opPrice::CALL => return 59,
        opPrice::RETURN => return 3,
        opPrice::STOP => return 0,
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
