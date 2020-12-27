extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

// mod main;
mod globals;

struct Vector {
	x: f64,
	y: f64
}

struct KeyState {
	left: bool,
	right: bool,
	jump: bool
}

struct Player {
	pos: Vector,
	vel: Vector,
	acc: Vector,
	size: Vector,
	color: [f32; 4],
	keys: KeyState,
  jump_time: i32,
  drift_time: i32,
  grounded: bool
}

struct Game {
	p1: Player,
	p2: Player
}

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

			// TODO: convert the game struct to a JSON and then send the JSON to the new client
			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				// TODO: whenever a message is received, it should contain the new state of the client that sent the action
				// need to update game state and broadcast new global state to all connected clients

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

fn process_keys(game: &mut Game, input: &Input) {
	match input {
    Input::Button(butargs) => {
    	match butargs.button {
        Button::Keyboard(Key::Space) => { 
        	if !game.p1.keys.jump && butargs.state == ButtonState::Press { 
        		game.p1.keys.jump = true;
        	}
        	if game.p1.keys.jump && butargs.state == ButtonState::Release {
        		game.p1.keys.jump = false;
        	} 
        },
        Button::Keyboard(Key::Left) => { 
        	if !game.p1.keys.left && butargs.state == ButtonState::Press { 
        		game.p1.keys.left = true; 
        	}
        	if game.p1.keys.left && butargs.state == ButtonState::Release {
        		game.p1.keys.left = false;
        	}
       	},
        Button::Keyboard(Key::Right) => {
        	if !game.p1.keys.right && butargs.state == ButtonState::Press { 
        		game.p1.keys.right = true; 
        	}
        	if game.p1.keys.right && butargs.state == ButtonState::Release {
        		game.p1.keys.right = false;
        	}
        },
        _ => {}
    }},
    _ => {}
  }
}

fn update(game: &mut Game, update_args: &UpdateArgs) {
  // rewrite for each player in game, update player
  update_player(&mut game.p1, update_args.dt);
  // update_player(&mut game.p2, update_args.dt);
}

fn update_player(p: &mut Player, dt: f64) {
	// determine accelerations based on states
	p.acc.y = -globals::GRAVITY;
	if p.keys.jump {
    if p.jump_time < globals::JUMP_DURATION {
      p.acc.y = globals::JUMP_FORCE;
      p.jump_time += 1;
    }
  } else if p.grounded {
    p.jump_time = 0;
  } else {
    p.jump_time = globals::JUMP_DURATION;
  }

	if p.keys.left { 
    if p.drift_time < globals::DRIFT_DURATION {
      p.acc.x = -globals::DRIFT_FORCE;
      p.drift_time += 1;
    }
  } else if p.keys.right {
    if p.drift_time < globals::DRIFT_DURATION {
      p.acc.x = globals::DRIFT_FORCE;
      p.drift_time += 1;
    }
  } else {
    if p.vel.x >= globals::MIN_XSPEED {
      if p.grounded {
        p.acc.x -= globals::GND_DRAG_FORCE;
      } else {
        p.acc.x -= globals::AIR_DRAG_FORCE;
      }
      if p.vel.x + p.acc.x < globals::MIN_XSPEED {
        p.vel.x = 0.0;
        p.acc.x = 0.0;
      }
    }
    if p.vel.x <= -globals::MIN_XSPEED {
      if p.grounded {
        p.acc.x += globals::GND_DRAG_FORCE;
      } else {
        p.acc.x += globals::AIR_DRAG_FORCE;
      }
      if p.vel.x + p.acc.x > -globals::MIN_XSPEED {
        p.vel.x = 0.0;
        p.acc.x = 0.0;
      }
    }
    p.drift_time = 0;
  }

	// integrate acceleration to get velocities
	p.vel.x += p.acc.x * dt;
	p.vel.y += p.acc.y * dt;

	// integrate velocities to get positions
	p.pos.x += p.vel.x * dt;
	p.pos.y += p.vel.y * dt;

  // do bounds checks (top and left)
  if p.pos.y <= globals::BOX_HEIGHT {  // floor
    p.pos.y = globals::BOX_HEIGHT + globals::MIN_YSPEED*dt;
    p.vel.y = 0.0;
  } else if p.pos.y >= globals::HEIGHT {  // ceiling
    p.pos.y = globals::HEIGHT - globals::MIN_YSPEED*dt;
    p.vel.y = 0.0;
  }
  if p.pos.x <= 0.0 {  // left
    p.pos.x = 0.0 + globals::MIN_XSPEED*dt;
    p.vel.x = 0.0;
  } else if p.pos.x >= globals::WIDTH {  // right
    p.pos.x = globals::WIDTH - globals::MIN_XSPEED*dt;
    p.vel.x = 0.0;
  }

  p.grounded = p.pos.y <= globals::BOX_HEIGHT + globals::MIN_YSPEED*dt;
  // println!("P[x:{}, y:{}, vx: {}, vy:{}, ax:{}, ay:{}]", p.pos.x, p.pos.y, p.vel.x, p.vel.y, p.acc.x, p.acc.y);

  // TODO: for all of these updates, send a message to server saying that the player's info has changed.
}