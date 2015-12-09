extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate secp256k1;
extern crate bincode;

use self::rand::{Rng, OsRng};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::secp256k1::*;
use self::secp256k1::key::*;
use self::bincode::SizeLimit;
use self::bincode::rustc_serialize::{encode, decode};

// Generate a string with UTF8 chars of variable length.
// Input    size        Length of generated string.
// Output   String      Generated string.
pub fn gen_string(size: i32)-> String{
    let mut rng = match rand::os::OsRng::new(){
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS Rng: {}", e)
    };
    let buf: String = rng.gen_ascii_chars().take(size as usize).collect();
    return buf;
}

// Hashes two strings using SHA256
// Input    a           First string to be hashed.
// Input    b           Second string to be hashed.
// Output   String      Final hash.
pub fn string_hash(a: &str, b: &str) -> String{
        let mut sha = Sha256::new();
        sha.input_str(a);
        sha.input_str(b);
        return sha.result_str().to_string();
}

// Hashes a string and a number using SHA256
// Input    a           String to be hashed.
// Input    b           Number to be hashed.
// Output   String      Final hash.
pub fn string_int_hash(a: &str, b: &i64) -> String{
        let mut sha = Sha256::new();
        sha.input_str(a);
        sha.input_str(&(b.to_string()));
        return sha.result_str().to_string();
}

// Generate a public key using SECP256K1 Elliptic Curve
// Input    sk          Corresponding secret key.
// Output   PublicKey   Generated public key.
pub fn gen_public_key(sk: &SecretKey)-> PublicKey{
    let engine = Secp256k1::new();
    PublicKey::from_secret_key(&engine, sk).unwrap()
}

// Generate a secret key using SECP256K1 Elliptic Curve
// Output   SecretKey   Generated secret key
pub fn gen_secret_key()-> SecretKey{
    let mut rng = match rand::os::OsRng::new(){
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS Rng: {}", e)
    };
    let engine = Secp256k1::new();
    SecretKey::new(&engine, &mut rng)
}

// Generate shared secret.
// Input    sk              Secret key
// Input    pk              Public key
// Output   SharedSecret    Generated SharedSecret
pub fn gen_shared_secret(pk: &PublicKey, sk: &SecretKey)->ecdh::SharedSecret{
    let engine = Secp256k1::new();
    ecdh::SharedSecret::new(&engine, pk, sk)
}

// Verifying public key.
// Input    pk          Public key
// Output   Boolean     Valid?
pub fn check_public_key(pk: &PublicKey) -> bool{
    pk.is_valid()
}

// Validate if signature for message uses the corresponding public key.
// Input    mess        Message to verify
// Input    sig         Signature
// Input    pk          Public key
// Output   Result      Valid?
pub fn check_message(mess: &Message, sig: &Signature, pk: &PublicKey) -> Result<(), Error>{
    let engine = Secp256k1::new();
    Secp256k1::verify(&engine, mess, sig, pk)
}

// Sign message using secret key.
// Input    data        Message to sign
// Input    sk          Secret Key
// Output   Result      Returns signature on success, error if not.
pub fn sign_message(data: &[u8], sk: &SecretKey) -> Result<Signature, Error>{
    let engine = Secp256k1::new();
    let mess: Message = Message::from_slice(data).unwrap();
    Secp256k1::sign(&engine, &mess, sk)
}

// Generate a message from string slice.
// Input    mess        Message to generate
// Output   Result      Returns message on success, error if not.
pub fn gen_message(mess: &str) -> Result<Message, Error>{
    return Message::from_slice(mess.as_bytes());
}

// Encode secret key to byte vector
// Input    sk          Secret key to encode
// Output   Vec<u8>     Converted byte vector
pub fn encode_sk(sk: &SecretKey)-> Vec<u8>{
    let secret_key: Vec<u8> = encode(&sk, SizeLimit::Infinite).unwrap();
    return secret_key;
}

// Decode byte vector to secret key
// Input    Vec<u8>      Byte vector to convert
// Output   SecretKey    Converted secret key
pub fn decode_sk(sk: &Vec<u8>)-> SecretKey{
    let secret_key: SecretKey = decode(&sk).unwrap();
    return secret_key;
}

// //Tests
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_krypto() {
//         println!("\n\n\nKrypto test");
//         // let a = "abcdefg";
//         // let b = "hijklmn";
//         // let c = string_hash(a, b);
//         // println!("Hash of [a,b] is: {}", c);
//
//         //Elliptical Curves
//         let secret_key = gen_secret_key();
//         let public_key = gen_public_key(&secret_key);
//         let shared_secret = gen_shared_secret(&public_key, &secret_key);
//         println!("\nSecret Key: {:?}\n", secret_key);
//         println!("\nPublic Key: {:?}\n", public_key);
//         println!("\nValid Public Key: {:?}\n", check_public_key(&public_key));
//         println!("\nShared Secret: {:?}\n", shared_secret);
//
//
//     }
// }
