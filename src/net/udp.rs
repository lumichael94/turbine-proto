// // Generic UDP Server Implementation
// use std::env;
// use std::net;
// use std::thread;
//
// fn socket(listen_addr: net::SocketAddr) -> net::UdpSocket {
// 	let bind = net::UdpSocket::bind(listen_addr);
// 	let mut socket;
// 	match bind {
// 		Ok(sock) => {
// 			println!("Socket bound to {}", listen_addr);
// 			socket = sock;
// 		}
// 		Err(err) => panic!("Could not bind: {}", err)
// 	}
// 	socket
// }
//
// fn read(socket: net::UdpSocket) -> Vec<u8> {
// 	let mut buffer: [u8; 1] = [0; 1];
// 	let result = socket.recv_from(&mut buffer);
// 	drop(socket);
// 	let mut data;
// 	match result {
// 		Ok((size, src)) => {
// 			data = Vec::from(&buffer[0..size])
// 		},
// 		Err(err) => panic!("Error while reading: {}", err)
// 	}
// 	data
// }
//
// pub fn send(send_addr: net::SocketAddr, destination: net::SocketAddr, data: Vec<u8>) {
// 	let socket = socket(send_addr);
// 	let result = socket.send_to(&data, destination);
// 	drop(socket);
// 	match result {
// 		Ok(size) => println!("Sent {} bytes of data", size),
// 		Err(err) => panic!("Error while sending: {}", err)
// 	}
// }
//
// pub fn listen(listen_addr: net::SocketAddr) -> thread::JoinHandle<Vec<u8>> {
// 	let socket = socket(listen_addr);
// 	let handle = thread::spawn(move || {
// 		read(socket)
// 	});
// 	handle
// }
//
// #[cfg(test)]
// mod test {
//   use std::net;
//   use std::thread;
//   use super::*;
//
//   #[test]
//   fn test_udp() {
//     println!("UDP server test");
//     let ip = net::Ipv4Addr::new(127, 0, 0, 1);
//     let listen_addr = net::SocketAddrV4::new(ip, 8888);
//     let send_addr = net::SocketAddrV4::new(ip, 8889);
//     let future = listen(net::SocketAddr::V4(listen_addr));
//     let message: Vec<u8> = vec![10];
//
//     thread::sleep_ms(1000);
//     send(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), message);
//     println!("Waiting");
//     let received = future.join().unwrap();
//     println!("Got {} bytes", received.len());
//     assert_eq!(1, received.len());
//     assert_eq!(10, received[0]);
//   }
// }
