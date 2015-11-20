// Generic TCP Server Implementation
use std::net;
use std::thread;
use std::time::Duration;

use std::io::Read;
use std::io::Write;


pub fn listen(listen_addr: net::SocketAddr) {
	let listener = net::TcpListener::bind(listen_addr).unwrap();
	println!("Listening started on {}", listen_addr);

	let ip = net::Ipv4Addr::new(127, 0, 0, 1);
	let send_addr = net::SocketAddrV4::new(ip, 8888);
	connect(send_addr);

    for stream in listener.incoming() {
    	match stream {
    		Err(e) => { println!("Error on listening: {}", e) }
    		Ok(stream) => {
    			thread::spawn(move || {
					//This only works on nightly it seems. Need to make the switch.
					// stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    				read_handle(stream);
    			});
    		}
    	}
	}
}

pub fn connect(connect_addr: net::SocketAddrV4) {
	let stream = net::TcpStream::connect(connect_addr).unwrap();
	write_handle(stream);
}

fn write_handle(mut stream: net::TcpStream){
	let mut output = send_info();
	match stream.write(&output) {
		Err(e) => println!("Error on writing to stream!"),
		Ok(_) => {
			if output[0] == 1 {
				let print_this: &[u8] = &output;
				println!("Output is: {:?}", print_this);
			} else {
				println!("Output is not formatted correctly.");
			}
		},
	}

}

fn read_handle(mut stream: net::TcpStream) {
	println!("Passed to handler");
	let mut buffer;
	let mut output = send_info();
	loop {
		buffer = [0; 4];

		let _ = match stream.read(&mut buffer) {
			Err(e) => panic!("Error on read: {}", e),
			Ok(_) => {
				if buffer[0] == 0 {
					break; 	// EOF
				} else {
					println!("Read the message: {:?}", buffer);
					break;
				}
			},
		};
	}
	println!("Finished reading from stream.");
}

fn send_info() -> [u8; 4]{
	let mut test_buffer: [u8; 4] = [1; 4]; //Change in the future
	return test_buffer;
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
