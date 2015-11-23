extern crate rand;
extern crate crypto;
extern crate rustc_serialize;

use vm::opCodes::{opCode, map_to_fuel};
use vm::opCodes::opCode::*;
use util::helper;

pub fn decode(code: &Vec<String>) -> (Vec<opCode>, Vec<Vec<String>>, Vec<i64>){
    let mut instr_vec:  Vec<opCode>     = Vec::new();
    let mut param_vec:  Vec<Vec<String>>  = Vec::new();
    let mut fuel_vec:   Vec<i64>        = Vec::new();

    for instr in code {
        let it = instr.split(" ");
        let coll: Vec<&str> = it.collect();

        //Operation Code
        let op_string: &str = coll[0];
        let op: opCode = map_to_instr(op_string);

        //Operation Parameters
        let mut param: Vec<&str> = Vec::new();
        for p in 1..coll.len(){
            param.push(coll[p]);
        }

        //Operation Fuel
        let fuel: i64 = map_to_opFuel(op_string);

        //Push values in
        instr_vec.push(op);
        param_vec.push(helper::vec_slice_to_string(&param));
        fuel_vec.push(fuel);
    }
    return (instr_vec, param_vec, fuel_vec);
}

pub fn map_to_instr(instr: &str) -> opCode{
    match instr {
        "ADD"   => return ADD,
        "SUB"   => return SUB,
        "MUL"   => return MUL,
        "DIV"   => return DIV,
        "MOD"   => return MOD,
        "LOAD"  => return LOAD,
        "PUSH"  => return PUSH,
        "POP"   => return POP,
        "STOP"  => return STOP,
        _       => return ERROR,
    }
}

pub fn map_to_string(code: &opCode) -> String{
    match code {
        &ADD   => return "ADD".to_string(),
        &SUB   => return "SUB".to_string(),
        &MUL   => return "MUL".to_string(),
        &DIV   => return "DIV".to_string(),
        &MOD   => return "MOD".to_string(),
        &LOAD  => return "LOAD".to_string(),
        &PUSH  => return "PUSH".to_string(),
        &POP   => return "POP".to_string(),
        &STOP  => return "STOP".to_string(),
        _       => return "ERROR".to_string(),
    }
}

pub fn map_to_opFuel(instr: &str) -> i64 {
    match instr {
        "ADD"   => return map_to_fuel(ADD),
        "SUB"   => return map_to_fuel(SUB),
        "MUL"   => return map_to_fuel(MUL),
        "DIV"   => return map_to_fuel(DIV),
        "MOD"   => return map_to_fuel(MOD),
        "LOAD"  => return map_to_fuel(LOAD),
        "PUSH"  => return map_to_fuel(PUSH),
        "POP"   => return map_to_fuel(POP),
        "STOP"  => return map_to_fuel(STOP),
        _       => return map_to_fuel(ERROR),
    }
}

// #[cfg(test)]
// mod test {
//   use super::*;
//   use vm::opCodes::{opCode, map_to_fuel};
//   use vm::opCodes::opCode::*;
//   use util::helper::vec_slice_to_string;
//
//   #[test]
//   fn test_decoder() {
//     println!("decoder test");
//     let code_arr: Vec<&str> = vec!["LOAD 1","LOAD 2", "POP 2", "ADD 2", "PUSH 1", "STOP"];
//     let code: Vec<String> = vec_slice_to_string(&code_arr);
//     let instr_set: (Vec<opCode>, Vec<Vec<String>>, Vec<i64>) = decode(&code);
//     let args: Vec<Vec<String>> = instr_set.1;
//     // let args: Vec<i64> = instr_set.2;
//     // println!("Instruction set is: {:?}", args);
//   }
// }
