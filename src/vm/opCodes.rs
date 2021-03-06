use vm::env::env;

// Enum for operation code
pub enum opCode {
    ADD, MUL, DIV, MOD, SUB,
    POP, LOAD, PUSH, STOP, ERROR,
    SEND, JUMP,
}

// Enum for operation function parameters
pub enum opCode_param<'a> {
    ADD(&'a mut env, i32),
    SUB(&'a mut env, i32),
    POP(&'a mut env, i32),
    PUSH(&'a mut env, i32),
    MUL(&'a mut env, i32),
    DIV(&'a mut env, i32),
    MOD(&'a mut env, i32),
    LOAD(&'a mut env, i32),
    SEND(&'a mut env, i32),
    JUMP(&'a mut env, i32),
    STOP(&'a mut env),
    ERROR(&'a mut env),
}

// Maps Operation code to corresponding function
// Input    code    Operation code
pub fn map_to_fn(code: opCode_param) {
    match code {
        opCode_param::ADD(env, n)      => add(env, n),
        opCode_param::SUB(env, n)      => sub(env, n),
        opCode_param::MUL(env, n)      => mul(env, n),
        opCode_param::DIV(env, n)      => div(env, n),
        opCode_param::MOD(env, n)      => modulo(env, n),

        opCode_param::LOAD(env, word)          => load(env, word),
        opCode_param::PUSH(env, n)     => push(env, n),
        opCode_param::POP(env, n)      => pop(env, n),
        opCode_param::SEND(env, n)    => send(env, n),
        opCode_param::JUMP(env, n)    => jump(env, n),
        opCode_param::STOP(env)       => stop(env),
        opCode_param::ERROR(env)      => error(env),

    };
}

// Operation function for ADD
// Input    env     Virtual machine environment
// Input    n       Number of words to add from stack
fn add(mut env: &mut env, n: i32){
    if env.env_log.fuel > map_to_fuel(opCode::SUB){
        let mut ans = env.origin.memory.pop().unwrap();
        env.env_log.fuel -= map_to_fuel(opCode::ADD);
        for _ in 0..(n-1){
            ans += env.origin.memory.pop().unwrap();
        }
        env.origin.memory.push(ans);
        println!("ADD: {}", env.origin.memory.last().unwrap());
    } else {
        env.origin.pc = -2;
    }
}

// Operation function for SUB
// Input    env     Virtual machine environment
// Input    n       Number of words to subtract from stack
fn sub(mut env: &mut env, n: i32){
    if env.env_log.fuel > map_to_fuel(opCode::SUB){
        env.env_log.fuel -= map_to_fuel(opCode::SUB);
        let mut ans = env.origin.memory.pop().unwrap();
        for _ in 0..(n-1){
            ans -= env.origin.memory.pop().unwrap();
        }
        env.origin.memory.push(ans);
        println!("SUB: {}", env.origin.memory.last().unwrap());
    } else {
        env.origin.pc = -2;
    }
}

// Operation function for MUL
// Input    env     Virtual machine environment
// Input    n       Number of words to multiply from stack
fn mul(mut env: &mut env, n: i32){
    if env.env_log.fuel > map_to_fuel(opCode::MUL){
        env.env_log.fuel -= map_to_fuel(opCode::MUL);
        let mut ans = env.origin.memory.pop().unwrap();
        for _ in 0..(n-1){
            ans *= env.origin.memory.pop().unwrap();
        }
        env.origin.memory.push(ans);
        println!("MUL: {}", env.origin.memory.last().unwrap());
    } else {
        env.origin.pc = -2;
    }

}

// Operation function for DIV
// Input    env     Virtual machine environment
// Input    n       Number of words to divide from stack
fn div(mut env: &mut env, n: i32){
    if env.env_log.fuel > map_to_fuel(opCode::DIV){
        env.env_log.fuel -= map_to_fuel(opCode::DIV);
        let mut ans = env.origin.memory.pop().unwrap();
        for _ in 0..(n-1){
            ans /= env.origin.memory.pop().unwrap();
        }
        env.origin.memory.push(ans);
        println!("DIV: {}", env.origin.memory.last().unwrap());
    } else {
        env.origin.pc = -2;
    }
}

// Operation function for MOD
// Input    env     Virtual machine environment
// Input    n       Number of words to modulo from stack
fn modulo(mut env: &mut env, n: i32){
    if env.env_log.fuel > map_to_fuel(opCode::MOD){
        env.env_log.fuel -= map_to_fuel(opCode::MOD);
        let mut ans = env.origin.memory.pop().unwrap();
        for _ in 0..(n-1){
            ans %= env.origin.memory.pop().unwrap();
        }
        env.origin.memory.push(ans);
        println!("MOD: {}", env.origin.memory.last().unwrap());
    } else {
        env.origin.pc = -2;
    }
}

// TODO: Implement sending fuel to target account.
// Operation function for SEND
// Input    env     Virtual machine environment
// Input    n       Fuel to send
fn send(mut env: &mut env, n: i32){
    if env.env_log.fuel > map_to_fuel(opCode::SEND){
        env.env_log.fuel -= map_to_fuel(opCode::SEND);
        env.env_log.fuel -= n as i64;
        println!("SEND: {}", env.origin.memory.last().unwrap());
    } else {
        env.origin.pc = -2;
    }
}

// Operation function for JUMP
// Input    env     Virtual machine environment
// Input    n       Operation number to jump to
fn jump(mut env: &mut env, n: i32){
    if env.env_log.fuel > map_to_fuel(opCode::JUMP){
        env.env_log.fuel -= map_to_fuel(opCode::JUMP);
        env.origin.pc = n as i64;
        println!("JUMP: {}", env.origin.memory.last().unwrap());
    } else {
        env.origin.pc = -2;
    }
}

// Operation function for POP
// Input    env     Virtual machine environment
// Input    n       Number of words to pop from stack into memory
fn pop(mut env: &mut env, n: i32){
    if env.env_log.fuel > (map_to_fuel(opCode::POP) * (n as i64)){
        env.env_log.fuel -= map_to_fuel(opCode::POP);
        for _ in 0..n{
            env.origin.memory.push(env.origin.stack.pop().unwrap());
        }
    } else {
        env.origin.pc = -2;
    }
    println!("POP")
}

// Operation function for POP
// Input    env     Virtual machine environment
// Input    n       Number of words to pop from memory into stack
fn push(mut env: &mut env, n: i32){
    if env.env_log.fuel > (map_to_fuel(opCode::PUSH) * (n as i64)){
        env.env_log.fuel -= map_to_fuel(opCode::PUSH);
        for _ in 0..n{
            env.origin.stack.push(env.origin.memory.pop().unwrap());
        }
        println!("PUSH")
    } else {
        env.origin.pc = -2;
    }
}

// Operation function for LOAD
// Input    env     Virtual machine environment
// Input    word    Word to load into stack
fn load(mut env: &mut env, word: i32){
    if env.env_log.fuel > map_to_fuel(opCode::LOAD){
        env.env_log.fuel -= map_to_fuel(opCode::LOAD);
        env.origin.stack.push(word as u8);
        println!("LOAD")
    } else {
        env.origin.pc = -2;
    }
}

// Operation function for STOP
// Input    env     Virtual machine environment
fn stop(mut env: &mut env){
    env.origin.pc = -1;
    println!("STOP");
}

// Operation function for ERROR
// Input    env     Virtual machine environment
fn error(mut env: &mut env){
    env.origin.pc = -2;
    println!("ERROR");
}

// Maps operation code to fuel
// Input    code    Operation code
// Output   i64     Fuel cost
pub fn map_to_fuel(code: opCode) -> i64{
    match code{
        opCode::ADD => return 1,
        opCode::SUB => return 1,
        opCode::MUL => return 5,
        opCode::DIV => return 5,
        opCode::MOD => return 13,
        opCode::POP => return 2,
        opCode::LOAD => return 7,
        opCode::SEND => return 10,
        // opPrice::STORE => return 11,
        // opPrice::STORE8 => return 31,
        opCode::JUMP => return 7,
        // opPrice::PC => return 23,
        // opPrice::FUEL => return 13,
        // opPrice::JUMPDEST => return 19,
        // opCode::ADDRESS => return 43,
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

// #[cfg(test)]
// mod test {
//   use super::*;
//
//   #[test]
//   fn test_opCodes() {
//     println!("opCode test");
//     // let x = map_to_fuel(opPrice::ADD);
//     // println!("ADD returns fuel of 1: {}", x);
//   }
// }
