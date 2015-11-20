extern crate rand;
extern crate crypto;
extern crate rustc_serialize;

use self::rand::{Rng, OsRng};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;


pub fn gen_add32()-> String{
    let mut rng = match rand::os::OsRng::new(){
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS Rng: {}", e)
    };

    let buf: String = rng.gen_ascii_chars().take(32).collect();
    return buf;
}

pub fn string_hash(a: &str, b: &str) -> String{
        let mut sha = Sha256::new();
        sha.input_str(a);
        sha.input_str(b);
        return sha.result_str().to_string();
}

// Tests
#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_krypto() {
    //     println!("Krypto test");
    //     let a = "abcdefg";
    //     let b = "hijklmn";
    //     let c = string_hash(a, b);
    //     println!("Hash of [a,b] is: {}", c);
    // }
}
