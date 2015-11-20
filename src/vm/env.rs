extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;

struct env_var {
    stack   : Vec<i32>,
    pc      : i64,
    memory  : Vec<i32>,
}

// pub fn main_loop(){
//
// }

#[cfg(test)]
mod test {
  use super::*;
  use super::env_var;
  use vm::opCodes::map_to_fn;
  // use vm::opCodes::opCode::{ADD, SUB, LOAD, PUSH, POP, MUL, DIV, MOD,
  //                           STOP,};
  use vm::opCodes::opCode::*;
  use vm::decoder;

  #[test]
  fn test_env() {
    println!("env test");

    let mut env = env_var{stack: Vec::new(), pc: 0, memory: Vec::new(),};
    let x: i32 = 7;
    let y: i32 = 2;
    map_to_fn(LOAD(&mut env.stack, x));
    map_to_fn(LOAD(&mut env.stack, y));
    map_to_fn(POP(&mut env.stack, &mut env.memory, 2));
    println!("\n\nLoaded variables");
    println!("This is the stack: {:?}", env.stack);
    println!("This is the memory: {:?}\n\n", env.memory);

    map_to_fn(MOD(&mut env.stack, &mut env.memory, 2));

    println!("This is the stack: {:?}", env.stack);
    println!("This is the memory: {:?}", env.memory);

    map_to_fn(PUSH(&mut env.stack, &mut env.memory, 1));

    println!("\n\nOperation finished.");
    println!("This is the stack: {:?}", env.stack);
    println!("This is the memory: {:?}", env.memory);

    map_to_fn(STOP(&mut env.pc));
    println!("This is the program counter: {:?}\n\n", env.pc);
  }
}
