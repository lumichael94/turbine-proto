// Generic TCP Server Implementation
extern crate postgres;


use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::{thread, str};
use std::time::Duration;

use std::io::Read;
use std::io::Write;

use data::account::account;
use data::database;
use network::proto;
use postgres::{Connection, SslMode};


// pub fn listen(listen_addr: SocketAddrV4) {
pub fn listen(address: &str) {
	let listener = TcpListener::bind(address).unwrap();
	println!("Listening started on {}", address);

    for stream in listener.incoming() {
    	match stream {
    		Err(e) => { println!("Error on listening: {}", e) }
    		Ok(stream) => {
    			thread::spawn(move || {
					//This only works on nightly it seems. Need to make the switch.
					// stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    				handle(stream);
    			});
    		}
    	}
	}
}

pub fn connect(address: &str){
	let mut stream = TcpStream::connect(address).unwrap();
	// write_handle(stream);
	handle(stream);
}

// fn write_handle(mut stream: TcpStream){
// 	let mut proto_code = send_info();
// 	match stream.write(&proto_code) {
// 		Err(e) => println!("Error on writing to stream!"),
// 		Ok(_) => {
// 			if proto_code[0..2] == [0,0] {
// 				let print_this: &[u8] = &proto_code;
// 				println!("Output is: {:?}", print_this);
// 			} else {
// 				println!("Output is not formatted correctly.");
// 			}
// 		},
// 	}
// }

fn handle(mut stream: TcpStream) {
	println!("Connected. Passed to handler");
	let conn = database::connect_db();
	let mut proto_buf;
	// proto::handshake(&mut stream, &conn);
	stream.write(&[2, 0]);
	loop {
		proto_buf = [0; 2];
		let _ = match stream.read(&mut proto_buf) {
			Err(e) => panic!("Error on read: {}", e),
			Ok(_) => match_proto(&proto_buf[..], &mut stream),
		};
	}
	println!("Finished reading from stream.");
	drop(stream);
	database::close_db(conn);
}

fn match_proto(incoming: &[u8], mut stream: &mut TcpStream){
	// println!("Received protocol: {:?}", incoming);
	match incoming[0]{
		0			=> println!("Goodbye"),
		1			=> println!("Go."),
		2			=> {
							println!("Incoming message >> Requesting Handshake");
							proto::send_handshake(stream);
						},
		3			=> {
							println!("Incoming message >> Sending Handshake");
							println!("Their account: {:?}", read_stream(stream, incoming[1]));
						},
		17 			=> println!("what is this."),
		_			=> println!("matches nothing.")
	}
}

fn read_stream(mut stream: &mut TcpStream, length: u8) -> Vec<u8>{
	let mut data_buf: Vec<u8> = Vec::new();
	let mut handle = stream.take(length as u64);
	let _ = handle.read(&mut data_buf);
		// Err(e) 	=> panic!("Error on read: {}", e),
		// Ok(_) 	=> {
	println!("a;lskdfj: {:?}", data_buf);
	return data_buf;
}


// #[cfg(test)]
// mod test {
//   use std::net;
//   use std::thread;
//   use super::*;
//
//   #[test]
//   fn test_tcp() {
//     println!("TCP server test");
//     let ip = net::Ipv4Addr::new(127, 0, 0, 1);
//     let listen_addr = net::SocketAddrV4::new(ip, 8888);
// 	let send_addr = net::SocketAddrV4::new(ip, 8888);
//     let listener = listen(net::SocketAddr::V4(listen_addr));
//   }
// }
