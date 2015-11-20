extern crate rand;
extern crate crypto;
extern crate rustc_serialize;

use vm::opCodes::opCode::*;
use vm::opCodes::*;

pub fn decoder(code: Vec<String>) {
    let instr_set: Vec<opCode> = Vec::new();

    for instr in code{
        let it = instr.split(" ");
        let coll: Vec<&str> = it.collect();
        let op = coll[0];
        let value = coll[1];

    }
}

pub fn match_to_instr<'a>(instr: String) -> opCode<'a>{
    match instr {
        "STOP"  => STOP(pc),
        "ADD"   => ADD(stack, memory, n),
        "STOP"  => SUB(stack, memory, n),
        "STOP"  => MUL(stack, memory, n),
        "STOP"  => DIV(stack, memory, n),
        "STOP"  => MOD(stack, memory, n),
        "STOP"  => LOAD(stack, word),
        "STOP"  => PUSH(stack, memory, n),
        "STOP"  => POP(stack, memory, n),
    }
}

#[cfg(test)]
mod test {
  use super::*;
  use super::env_var;
  use vm::opCodes::map_to_fn;
  use vm::opCodes::opCode::*;

  #[test]
  fn test_decoder() {
    println!("decoder test");
    let code: Vec<String> = vec!["LOAD 1","LOAD 2", "POP 2", "ADD 2", "PUSH 1", "STOP"];
  }
}
