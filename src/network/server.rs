use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::{thread, str};
use std::time::Duration;
use std::io::{Read, Write};

use data::account::account;
use data::database;
use network::proto;
use postgres::{Connection, SslMode};
use util::{helper, krypto};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;

pub fn listen(address: String, to_main: Sender<String>, connected: Arc<Mutex<HashMap<String, Sender<String>>>>){
	let add: &str  = &address;
	let listener = TcpListener::bind(add).unwrap();
	println!("\nListening started on {}", address);
	let _ = to_main.send("bound".to_string());

    for stream in listener.incoming() {
    	match stream {
    		Err(e) => { println!("Error on listening: {}", e) }
    		Ok(stream) => {
				let tx = to_main.clone();
				let (to_threads, from_main): (Sender<String>, Receiver<String>) = mpsc::channel();
				let mut arc = connected.clone();

    			thread::spawn(move || {
					let _ = stream.set_read_timeout(Some(Duration::new(5,0)));
    				handle(stream, tx, from_main, arc);
    			});
    		}
    	}
	}
}

//Connect to IP address.
pub fn connect(address: &str, to_main: Sender<String>,
		arc:  Arc<Mutex<HashMap<String, Sender<String>>>>){
	//Downstream, 1 to 1
	let (to_threads, from_main): (Sender<String>, Receiver<String>) = mpsc::channel();
	let stream_attempt = TcpStream::connect(address);
	match stream_attempt {
		Ok(stream) => {
			thread::spawn(move||{
				handle(stream, to_main, from_main, arc);
			});
		},
		Err(_) => {
			println!("Error connecting to peer: {:?}", address);
		}
	}
}

fn handle(mut stream: TcpStream, to_main: Sender<String>, from_main: Receiver<String>,
	arc:  Arc<Mutex<HashMap<String, Sender<String>>>>) {

	println!("Connected. Passed to handler");
	let mut proto_buf;
	let conn = database::connect_db();


	// Handshake
	let h_arc = arc.clone();
	let h_tx = to_main.clone();
	let attempt = proto::handshake(&mut stream, &conn, h_tx, h_arc);
	if attempt == None {
		return;
	}
	// Records node address for communicating with main.
	let address = attempt.unwrap();

	// Main handler loop
	loop {
		let connected = arc.clone();
		let tx = to_main.clone();
		match from_main.try_recv() {
			Ok(m) => {
				println!("Message received from main: {:?}", m);
				break;
			},
			Err(_)	=> {
				println!("No message received from main");
				proto_buf = [0; 2];
				let _ = match stream.read(&mut proto_buf) {
					Err(e) 	=> panic!("Error on read: {}", e),
					Ok(_) 	=> match_proto(&proto_buf[..], &mut stream, &conn, tx, connected),
				};
			},
		}
	}
	//Finish and exit
	println!("Finished reading from stream.");
	database::close_db(conn);
	drop(stream);
}

pub fn ping(stream: &mut TcpStream)-> bool{
	let mut inc = [0;2];
	let _ = stream.write(&[0, 0]);
	let b: bool = match stream.read(&mut inc){
		Err(_) => false,
		Ok(_) => {
			if inc[0] == 1 {
				return true;
			} else {
				return false;
			}
		},
	};
	return b;
}

fn match_proto(incoming: &[u8], mut stream: &mut TcpStream, conn: &Connection,
	to_main: Sender<String>, arc:  Arc<Mutex<HashMap<String, Sender<String>>>>){
	match incoming[0]{
		0 => {
			//No Response. Wait, then ping.
			let _ = stream.write(&[1,0]);
			thread::sleep(Duration::from_millis(500));
		},
		1 => {
			println!("Incoming message >> Ping");
			//Sending Pong
			//TODO: Should only send pong if not blacklisted.
			let _ = stream.write(&[1,0]);
			thread::sleep(Duration::from_millis(500));
		},
		2 => {
			println!("Incoming message >> Pong");
			//Sending Pong
			//TODO: Should only send pong if not blacklisted.
			let _ = stream.write(&[2,0]);
			thread::sleep(Duration::from_millis(500));
		},
		3 => {
			println!("Incoming message >> Requesting ");
			// println!("Incoming message >> Requesting Handshake");
			// proto::send_handshake(stream, conn);
		},
		4 => {
			println!("This should not be reached: 4");
			// println!("Incoming message >> Sending Handshake");
			// println!("Their handshake: {:?}", read_stream(stream, incoming[1]));
		},
		5 => {
			println!("Incoming message >> Requesting Logs");
			let raw_hash = read_stream(stream, incoming[1]);
			let hash = String::from_utf8(raw_hash).unwrap();
			proto::send_log(stream, hash, conn);
		},
		6 => {
			println!("Incoming message >> Sending Logs");
			println!("Their logs: {:?}", read_stream(stream, incoming[1]));
		},
		7 => {
			println!("Incoming message >> Requesting Account");
			let raw_address = read_stream(stream, incoming[1]);
			let address = String::from_utf8(raw_address).unwrap();
			proto::send_account(stream, address, conn);
		},
		8 => {
			println!("Incoming message >> Sending Account");
			println!("Their account: {:?}", read_stream(stream, incoming[1]));
		},
		9 => {
			println!("Incoming message >> Requesting State");
			let raw_hash = read_stream(stream, incoming[1]);
			let hash = String::from_utf8(raw_hash).unwrap();
			proto::send_state(stream, hash, conn);
		},
		10 => {
			println!("Incoming message >> Sending State");
			println!("Their state: {:?}", read_stream(stream, incoming[1]));
		},

		// Peer is requesting local status
		11 => {
			println!("Incoming message >> Requesting Status");
			// Asking consensus thread
			// Sending Status
		}
		12 => {
			println!("Incoming message >> Sending Status");
			let status: String = String::from_utf8(read_stream(stream, incoming[1])).unwrap();
		}
		_  => println!("matches nothing."),
	}
}

pub fn read_stream(stream: &mut TcpStream, length: u8) -> Vec<u8>{
	let mut data_buf = vec![0; length as usize];
	let _ = stream.read(&mut data_buf[..]);
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
