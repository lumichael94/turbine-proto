use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::{thread, str};
use std::time::Duration;
use std::io::{Read, Write};

use data::account::account;
use data::{database, node, log};
use network::proto;
use postgres::{Connection, SslMode};
use util::{helper, krypto};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;

pub fn listen(address: String, main_stat: Arc<RwLock<(String, String)>>,
nodes_stat: Arc<RwLock<HashMap<String, node::node>>>, curr_accs: Arc<RwLock<HashMap<String, account>>>,
    curr_logs: Arc<RwLock<HashMap<String, log::log>>>){
	let add: &str  = &address;
	let listener = TcpListener::bind(add).unwrap();
	println!("\nListening started on {}", address);

    for stream in listener.incoming() {
		//Cloning to move into threads
		let m_stat = main_stat.clone();
		let n_stat = nodes_stat.clone();
		let c_accs = curr_accs.clone();
		let c_logs = curr_logs.clone();

    	match stream {
    		Err(e) => { println!("Error on listening: {}", e) }
    		Ok(stream) => {
				// let tx = to_main.clone();
				let (to_threads, from_main): (Sender<String>, Receiver<String>) = mpsc::channel();
				// let mut arc = connected.clone();

    			thread::spawn(move || {
					let _ = stream.set_read_timeout(Some(Duration::new(5,0)));
    				handle(stream, m_stat, n_stat, c_accs, c_logs);
    			});
    		}
    	}
	}
}

//Connect to IP address.
pub fn connect(address: &str, main_stat: Arc<RwLock<(String, String)>>,
nodes_stat: Arc<RwLock<HashMap<String, node::node>>>, curr_accs: Arc<RwLock<HashMap<String, account>>>,
    curr_logs: Arc<RwLock<HashMap<String, log::log>>>){
	//Downstream, 1 to 1
	let (to_threads, from_main): (Sender<String>, Receiver<String>) = mpsc::channel();
	let stream_attempt = TcpStream::connect(address);
	match stream_attempt {
		Ok(stream) => {
			thread::spawn(move||{
				handle(stream, main_stat, nodes_stat, curr_accs, curr_logs);
			});
		},
		Err(_) => {
			println!("Error connecting to peer: {:?}", address);
		}
	}
}

fn handle(mut stream: TcpStream, main_stat: Arc<RwLock<(String, String)>>,
nodes_stat: Arc<RwLock<HashMap<String, node::node>>>, curr_accs: Arc<RwLock<HashMap<String, account>>>,
    curr_logs: Arc<RwLock<HashMap<String, log::log>>>) {

	println!("Connected. Passed to handler");
	let mut proto_buf;
	let conn = database::connect_db();

	// Handshake
	let hs_mstat = main_stat.clone();
    let hs_nstat = nodes_stat.clone();
	let attempt = proto::handshake(&mut stream, &conn, hs_mstat, hs_nstat);
	if attempt == None {
		return;
	}
	// Appending node into node status arc.
	let node_hs = attempt.unwrap();
    let node_add = node_hs.address;
    let node_acc = node_hs.account;

    // This is mutable for further updating.
    let mut nde = node::node{
        status:     node_hs.status,
        t_status:   "READY".to_string(),
        acc_hash:   node_add.clone(),
        s_hash:     node_acc.state,
        s_nonce:    node_acc.s_nonce,
    };

    let hs_arc = nodes_stat.clone();
    hs_arc.write().unwrap().insert(node_add, nde.clone());

	// Main handler loop
	loop {
        // TODO: Any method to prevent this chain cloning and passing of arcs?
        //Cloning to move into protocol matching and execution
		let m_stat = main_stat.clone();
		let n_stat = nodes_stat.clone();
		let c_accs = curr_accs.clone();
		let c_logs = curr_logs.clone();

		proto_buf = [0; 2];
		let _ = match stream.read(&mut proto_buf) {
			Err(e) 	=> panic!("Error on read: {}", e),

			Ok(_) 	=> match_proto(&proto_buf[..], &mut stream, &conn, m_stat, n_stat, c_accs, c_logs),
		};
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
	local_stat: Arc<RwLock<(String, String)>>, nodes_stat: Arc<RwLock<HashMap<String, node::node>>>,
	curr_accs: Arc<RwLock<HashMap<String, account>>>, curr_logs: Arc<RwLock<HashMap<String, log::log>>>){
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
