use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::{thread, str};
use std::time::Duration;
use std::io::{Read, Write};
use data::{database, tenv, log};
use network::proto::*;
use postgres::{Connection, SslMode};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub fn listen(address: String, main_stat: Arc<RwLock<(String, String)>>,
nodes_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>, curr_logs: Arc<RwLock<HashMap<String, log::log>>>){
	let add: &str  = &address;
	let listener = TcpListener::bind(add).unwrap();
	println!("\n=>> Listening started on {}", address);

    for stream in listener.incoming() {
		//Cloning to move into threads
		let m_stat = main_stat.clone();
		let n_stat = nodes_stat.clone();
		// let c_accs = curr_accs.clone();
		let c_logs = curr_logs.clone();

    	match stream {
    		Err(e) => { println!("=>> Error on listening: {}", e) }
    		Ok(stream) => {
    			thread::spawn(move || {
					let _ = stream.set_read_timeout(Some(Duration::new(5,0)));
    				handle(stream, m_stat, n_stat, c_logs);
    			});
    		}
    	}

	}
}

//Connect to IP address.
pub fn connect(address: &str, main_stat: Arc<RwLock<(String, String)>>,
nodes_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>, curr_logs: Arc<RwLock<HashMap<String, log::log>>>){

	let stream_attempt = TcpStream::connect(address);
	match stream_attempt {
		Ok(stream) => {
			thread::spawn(move||{
				handle(stream, main_stat, nodes_stat, curr_logs);
			});
		},
		Err(_) => {
			println!("=>> Error connecting to peer: {:?}", address);
		}
	}
}

fn handle(mut stream: TcpStream, main_stat: Arc<RwLock<(String, String)>>,
nodes_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>, curr_logs: Arc<RwLock<HashMap<String, log::log>>>) {
	thread::sleep(Duration::from_millis(100));
	// println!("Connected. Passed to handler");
	let conn = database::connect_db();
    // Main Statuses
    let listening: String  = "LISTENING".to_string();
    let proposing: String  = "PROPOSING".to_string();
    let committing: String = "COMMITTING".to_string();

    // Handshake
    let hs_mstat = main_stat.clone();
    let hs_nstat = nodes_stat.clone();

    let attempt = request_handshake(&mut stream, &conn, hs_mstat, hs_nstat);
    let mut te;
    if attempt == None{
        return;                  // If handshake fails, return.
    } else {
        te = attempt.unwrap();
    }

    // Main handler loop
	loop {
        // Cloning arcs
        let main_arc = main_stat.clone();
        let logs_arc = curr_logs.clone();
        let marc = main_arc.clone();
		let (m_status, m_state) = get_main_stat(marc);
        if m_status == listening {
            // Exchange new logs
            let thread_status: String = te.t_stat;
            match &thread_status[..]{
                "BEHIND" => {
                    // Requesting Logs
                    let before_state = m_status.clone();
                    let next_shash: String = request_state_after(&mut stream, before_state).hash;
                    let req_l_arc = logs_arc.clone();
                    request_logs(&mut stream, req_l_arc, next_shash);
                },
                "SYNCED" => {
                    // Thread sleep. Wait for other threads to catch up.
                    thread::sleep(Duration::from_millis(500));
                },
                _        => println!("=>> Error on reading the thread status!"),
            }
        } else if m_status == proposing{
            // Thread broadcasts that it is synced and should be counted toward proposals
            // Requesting Logs

			let thread_status: String = te.t_stat.clone();

			match &thread_status[..]{
				"BEHIND" =>{
					// Do nothing. Do not add to the possible logs.
					//thread::sleep(Duration::from_millis(500));
					continue;
				},
				"SYNCED" =>{

					// println!("PROPOSING AND SYNCED");
					// Change thread status from SYNCED to NOTREADY
					te.t_stat = "NOTREADY".to_string();
					let larc = logs_arc.clone();
					let lmap = larc.read().unwrap().clone();
					send_poss_logs(&mut stream, lmap);
					let their_logs = request_poss_logs(&mut stream);
					// Comparing difference in logs.
					let diff = compare_logs(their_logs, larc);
					if diff == 0 {
						te.t_stat = "READY".to_string();
					}
				},
				_ 	=>{},
			}

        } else if m_status == committing{
            // Query connected node for current state hashes.
            let node_poss_hash: String = request_poss_shash(&mut stream);

            if node_poss_hash == m_state {
                te.t_stat = "SYNCED".to_string();
                let mut n_hmap = nodes_stat.write().unwrap();
                let updated_nde = te.clone();
                (*n_hmap).insert(te.n_add, updated_nde);
				drop(n_hmap);
            }
        } else {
            println!("=>> Error reading local status: {:?}", &m_status);
            break;
        }
        send_update(&mut stream, &conn, m_status);
        te = request_update(&mut stream, &conn);
        let node_add = te.n_add.clone();
        let tenv_arc = nodes_stat.clone();
        let mut tenv_writer = tenv_arc.write().unwrap();
        (*tenv_writer).insert(node_add, te.clone());
		drop(tenv_writer);
        // // Execute incoming
        // buf = [0; 2];
        // let _ = stream.read(&mut buf);
        // conn_proto(&mut buf, &mut stream, &conn, main_arc, nodes_arc, current_logs_arc);
    }
	// Finish and exit
	println!("=>> Finished reading from stream.");
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

fn conn_proto(incoming: &[u8], mut stream: &mut TcpStream, conn: &Connection,
	main_stat: Arc<RwLock<(String, String)>>, nodes_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>,
	curr_logs: Arc<RwLock<HashMap<String, log::log>>>){
	match incoming[0]{
		0 => {
			//No Response. Ping, then sleep.
			let _ = stream.write(&[1,0]);
			thread::sleep(Duration::from_millis(500));
		},
		1 => {
			println!("Incoming message >> Ping");
			//Sending Ping back
			let _ = stream.write(&[1,0]);
			thread::sleep(Duration::from_millis(500));
		},
		2 => {
            println!("Incoming message >> Requesting Handshake");
            let stat_arc = main_stat.clone();
            let stat_tup = stat_arc.read().unwrap();
            let status = stat_tup.0.clone();
            send_handshake(stream, conn, status.to_string());
		},
		3 => {
			println!("Incoming message >> Requesting Update");
            let stat_arc = main_stat.clone();
            let stat_tup = stat_arc.read().unwrap();
            let status = stat_tup.0.clone();
            send_update(&mut stream, conn, status);
		},
		4 => {
			println!("Incoming message >> Requesting Possible Logs");
            let arc_logs = curr_logs.clone();
            let logs_map = arc_logs.read().unwrap().clone();
            send_poss_logs(&mut stream, logs_map);
		},
        5 => {
            println!("Incoming message >> Requesting Possible State Hash");
            let stat_arc = main_stat.clone();
            let stat_tup = stat_arc.read().unwrap();
            let poss_state: String = stat_tup.1.clone();
            let raw_hash = poss_state.as_bytes();
            let size = raw_hash.len();
            let _ = stream.write(&[6, size as u8] );
            let _ = stream.write(&raw_hash);
        },
        6 => {

        }

		// 5 => {
		// 	println!("Incoming message >> Requesting Logs");
		// 	let raw_hash = read_stream(stream, incoming[1]);
		// 	let hash = String::from_utf8(raw_hash).unwrap();
		// 	proto::send_log(stream, hash, conn);
		// },
		// 6 => {
		// 	println!("Incoming message >> Sending Logs");
		// 	println!("Their logs: {:?}", read_stream(stream, incoming[1]));
		// },
		// 7 => {
		// 	println!("Incoming message >> Requesting Account");
		// 	let raw_address = read_stream(stream, incoming[1]);
		// 	let address = String::from_utf8(raw_address).unwrap();
		// 	proto::send_account(stream, address, conn);
		// },
		// 8 => {
		// 	println!("Incoming message >> Sending Account");
		// 	println!("Their account: {:?}", read_stream(stream, incoming[1]));
		// },
		// 9 => {
		// 	println!("Incoming message >> Requesting State");
		// 	let raw_hash = read_stream(stream, incoming[1]);
		// 	let hash = String::from_utf8(raw_hash).unwrap();
		// 	proto::send_state(stream, hash, conn);
		// },
		// 10 => {
		// 	println!("Incoming message >> Sending State");
		// 	println!("Their state: {:?}", read_stream(stream, incoming[1]));
		// },
        //
		// // Peer is requesting local status
		// 11 => {
		// 	println!("Incoming message >> Requesting Status");
		// 	// Asking consensus thread
		// 	// Sending Status
		// }
		// 12 => {
		// 	println!("Incoming message >> Sending Status");
		// 	let status: String = String::from_utf8(read_stream(stream, incoming[1])).unwrap();
		// }
        // 13 => { // Node is in proposing state
        //     println!("Incoming message >> Requesting Possible Hash.");
        //     let hash_string: String = local_stat.read().unwrap().1.clone();
        //     let hash_vec = hash_string.as_bytes();
        //     // Sending hash
        //     let _ = stream.write(&[14, hash_vec.len() as u8]);
        //     let _ = stream.write(hash_vec);
        // }
        // 14 => { // Node is in proposing state
        //     println!("Incoming message >> Sending Current Hash.");
        //
        // }
		_  => println!("matches nothing."),
	}
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
