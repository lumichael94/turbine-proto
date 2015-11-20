extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;

use vm::opCodes::{opCode, opCode_param, map_to_fuel, map_to_fn};
use vm::opCodes::opCode::*;
use vm::decoder::{decode, map_to_string};

pub struct env_state {
    stack   : Vec<i32>,
    pc      : i64,
    memory  : Vec<i32>,
}

//TODO: Implement
// pub fn init_from_slice(){
//
// }

pub fn execute_code(mut state: &mut env_state, instr_set: &(Vec<opCode>, Vec<Vec<&str>>, Vec<i64>)){

    loop {

        let ref instr: opCode = (instr_set.0)[state.pc as usize];
        let ref param: Vec<&str> = (instr_set.1)[state.pc as usize];
        let ref fuel_cost: i64 = (instr_set.2)[state.pc as usize];

        println!("\n\nExecute opCode: {}", map_to_string(&instr));
        println!("With params of: {:?}", param);
        println!("And a fuel cost of: {:?}", fuel_cost);

        execute_instr(&instr, &param, state);

        println!("This is the stack: {:?}", state.stack);
        println!("This is the memory: {:?}\n\n", state.memory);
        println!("This is the counter: {:?}", state.pc);

        if state.pc < 0 {
            break;
        }

        //Cleanup
        state.pc+=1;
    }
}

pub fn execute_instr(instr: &opCode, param: &Vec<&str>, mut state: &mut env_state){
    // map_to_fn();
    match instr {
        &ADD     => {
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::ADD(&mut state.stack, &mut state.memory, n));
        },
        &MUL     => {
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::MUL(&mut state.stack, &mut state.memory, n));
        },
        &DIV     => {
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::DIV(&mut state.stack, &mut state.memory, n));
        },
        &MOD     =>{
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::MOD(&mut state.stack, &mut state.memory, n));
        },
        &SUB     => {
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::SUB(&mut state.stack, &mut state.memory, n));
        },
        &POP     => {
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::POP(&mut state.stack, &mut state.memory, n));
        },
        &LOAD    =>{
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::LOAD(&mut state.stack, n));
        },
        &PUSH    =>{
            let n: i32 = param[0].to_string().parse::<i32>().unwrap();
            map_to_fn(opCode_param::PUSH(&mut state.stack, &mut state.memory, n));
        },
        &STOP    =>{
            map_to_fn(opCode_param::STOP(&mut state.pc));
        },
        &ERROR   => {
            map_to_fn(opCode_param::ERROR(&mut state.pc));
        },
    }
}


#[cfg(test)]
mod test {
  use super::*;
  use vm::opCodes::{opCode, opCode_param, map_to_fn};
  use vm::opCodes::opCode::*;
  use vm::decoder::decode;

  #[test]
  fn test_env1() {
    println!("\n\n\n\nenv test 1"); //Works!

    let mut env = env_state{stack: Vec::new(), pc: 0, memory: Vec::new(),};
    let x: i32 = 7;
    let y: i32 = 2;
    map_to_fn(opCode_param::LOAD(&mut env.stack, x));
    map_to_fn(opCode_param::LOAD(&mut env.stack, y));
    map_to_fn(opCode_param::POP(&mut env.stack, &mut env.memory, 2));
    println!("\n\nLoaded variables");
    println!("This is the stack: {:?}", env.stack);
    println!("This is the memory: {:?}\n\n", env.memory);

    map_to_fn(opCode_param::MOD(&mut env.stack, &mut env.memory, 2));

    println!("This is the stack: {:?}", env.stack);
    println!("This is the memory: {:?}", env.memory);

    map_to_fn(opCode_param::PUSH(&mut env.stack, &mut env.memory, 1));

    println!("\n\nOperation finished.");
    println!("This is the stack: {:?}", env.stack);
    println!("This is the memory: {:?}", env.memory);

    map_to_fn(opCode_param::STOP(&mut env.pc));
    println!("This is the program counter: {:?}\n\n", env.pc);
  }

  #[test]
  fn test_execute_code() {
    println!("\n\n\n\n\n\nenv test 2");
    let mut env = env_state{stack: Vec::new(), pc: 0, memory: Vec::new(),};
    let code: Vec<&str> = vec!["LOAD 1","LOAD 2", "POP 2", "ADD 2", "PUSH 1", "STOP"];
    let instr_set: (Vec<opCode>, Vec<Vec<&str>>, Vec<i64>) = decode(code);
    // let args: Vec<Vec<&str>> = instr_set.1;
    // println!("Instruction set is: {:?}", args);

    execute_code(&mut env, &instr_set);
  }
}
