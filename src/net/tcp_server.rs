extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn start_server(){
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    server_loop(listener);
}

fn server_loop(listener: TcpListener){
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                thread::spawn(move ||{
                    handle_node(stream);
                });
            }
            Err(e) => {
                println!("Error connecting to listener.");
            }
        }
    }
}

fn handle_node(stream: TcpStream){

}

fn stop_server(){

}

fn connect_node(){

}

fn disconnect_node(){

}

fn check_timeout(){

}

fn connected_nodes(){

}
