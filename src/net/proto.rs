extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;
use std::net::{TcpListener, TcpStream};
use std::thread;

// fn udp_connect(){
//
// }
//
// fn udp_disconnect(){
//
// }

fn tcp_connect(node_address: &str) -> TcpListener{
    // TODO: Check if node is blacklisted
    let listener = TcpListener::bind(node_address).unwrap();
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                thread::spawn(move||{

                });
            }
            Err(e) => {
                //  Problem connecting to network
            }
        }
    }
    // TODO: Write node to database
    return listener;

}

fn tcp_disconnect(listener:TcpListener){
    drop(listener);
}

//  Not implemented for sprint 3
// fn whisper_connect(){}
//
// fn whisper_disconnect(){}

//  Send logs of a block, usually called for the current block
fn send_logs(){

}

//  Send hash of a block
fn send_block_hash(){

}

fn request_block_receipt(){

}

fn request_log_receipt(){

}

fn send_block_id(){

}
