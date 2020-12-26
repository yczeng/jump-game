extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

// mod main;
mod globals;

fn main() {
	let server = Server::bind(globals::IP).unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(|| {
			if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
			}

			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			// let p1 = main::Player {
			//   	pos: Vector{x:2.0*globals::BOX_WIDTH, y:2.0*globals::BOX_HEIGHT+0.1},
			//   	vel: Vector{x:0.0, y:0.0},
			//   	acc: Vector{x:0.0, y:0.0},
			//   	size: Vector{x:2.0*globals::BOX_WIDTH, y:2.0*globals::BOX_HEIGHT},
			//   	color: [1.0, 0.0, 0.0, 1.0],
			//   	keys: KeyState{left: false, right: false, jump: false},
			//     jump_time: 0,
			//     drift_time: 0,
			//     grounded: false
			// };

			for message in receiver.incoming_messages() {
				// whenever a message is received, it should contain the new state of the client that sent the action
				// need to update state and broadcast new global state to all connected clients

				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					_ => sender.send_message(&message).unwrap(),
				}
			}
		});
	}
}