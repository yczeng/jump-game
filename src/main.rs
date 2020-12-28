extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
use piston_window::{Event, Input, Button, ButtonState, Key, Loop, UpdateArgs, RenderArgs};
use piston_window::{clear, rectangle};

extern crate websocket;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;
use std::net::TcpStream;

use websocket::client::ClientBuilder;
use websocket::sync::Client;
use websocket::sender::Writer;
use websocket::{Message, OwnedMessage};

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
	// p2: Player
}

fn main() {
  let mut window: PistonWindow =
    WindowSettings::new("Jump Game", [globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT])
    .exit_on_esc(true).build().unwrap();

  let p1 = Player {
  	pos: Vector{x:2.0*globals::BOX_WIDTH, y:2.0*globals::BOX_HEIGHT+0.1},
  	vel: Vector{x:0.0, y:0.0},
  	acc: Vector{x:0.0, y:0.0},
  	size: Vector{x:2.0*globals::BOX_WIDTH, y:2.0*globals::BOX_HEIGHT},
  	color: [1.0, 0.0, 0.0, 1.0],
  	keys: KeyState{left: false, right: false, jump: false},
    jump_time: 0,
    drift_time: 0,
    grounded: false
  };
  
  let mut game = Game {
  	p1: p1,
  	// p2: p2
  };

  // connect to web socket
  println!("Connecting to {}", globals::CONNECTION);

  let client = ClientBuilder::new(globals::CONNECTION)
    .unwrap()
    .add_protocol("rust-websocket")
    .connect_insecure()
    .unwrap();

  println!("Successfully connected");

  let (mut receiver, mut sender) = client.split().unwrap();
  let (tx, rx) = channel();
  let tx_1 = tx.clone();

  let receive_loop = thread::spawn(move || {
    // Receive loop
    for message in receiver.incoming_messages() {
      let message = match message {
        Ok(m) => m,
        Err(e) => {
          println!("Error receiving data from server: {:?}", e);
          let _ = tx_1.send(OwnedMessage::Close(None));
          return;
        }
      };

      match message {
        OwnedMessage::Close(_) => {
          // Got a close message, so send a close message and return
          let _ = tx_1.send(OwnedMessage::Close(None));
          println!("Closing connection to server");
          return;
        }
        OwnedMessage::Ping(data) => {
          match tx_1.send(OwnedMessage::Pong(data)) {
            // Send a pong in response
            Ok(()) => (),
            Err(e) => {
              println!("Pinging server failed: {:?}", e);
              return;
            }
          }
        }
        _ => { // update the local game state to the one we just received
          // game = parse_game(&message);
          println!("received: {:?}", message);
        },
      }
    }
  });

  // Whenever we get a key input from the user, send it to the server
  // 60 times a second, render the current game state stored in Game struct
  while let Some(event) = window.next() {
    match event {
      Event::Input(input_args, _timestamp) => { send_key(&mut game, &input_args, &mut sender); },
      Event::Loop(loop_args) => {
        match loop_args {
          Loop::Render(render_args) => { render(&game, &mut window, event, &render_args); },
          _ => {}
        }},
      _ => {}
    }
  }
}

fn send_key(game: &mut Game, input: &Input, sender: &mut Writer<TcpStream>) {

  match input {
    Input::Button(butargs) => {
      match butargs.button {
        Button::Keyboard(Key::Space) => {
          let mut keystate = "";
          if butargs.state == ButtonState::Press { 
            keystate = "space-press";
          } else if butargs.state == ButtonState::Release {
            keystate = "space-release";
          }
          let message = OwnedMessage::Text(keystate.to_string());
          sender.send_message(&message);
          println!("tred to send: {}", keystate);
        },
        Button::Keyboard(Key::Left) => { 
          let mut keystate = "";
          if butargs.state == ButtonState::Press { 
            keystate = "left-press";
          } else if butargs.state == ButtonState::Release {
            keystate = "left-release";
          }
          let message = OwnedMessage::Text(keystate.to_string());
          sender.send_message(&message);
          println!("tred to send: {}", keystate);
        },
        Button::Keyboard(Key::Right) => {
          let mut keystate = "";
          if butargs.state == ButtonState::Press { 
            keystate = "right-press";
          } else if butargs.state == ButtonState::Release {
            keystate = "right-release";
          }
          let message = OwnedMessage::Text(keystate.to_string());
          sender.send_message(&message);
          println!("tred to send: {}", keystate);
        },
        _ => {}
    }},
    _ => {}
  }
}

fn render(game: &Game, window: &mut PistonWindow, event: Event, render_args: &RenderArgs){
	let p1 = &game.p1;
	// let p2 = &game.p2;

	window.draw_2d(&event, |c, g, _d| {
    clear([0.0, 0.0, 0.0, 1.0], g);	 // clear the screen
    rectangle(p1.color,	to_window_coords(&p1), c.transform, g);  // draw player 1
    // rectangle(p2.color, to_window_coords(&p2), c.transform, g);  // draw player 2
  });
}

fn to_window_coords(p: &Player) -> [f64; 4] {
  return [(p.pos.x - globals::BOX_WIDTH)*globals::WINDOW_SCALE,
          globals::WINDOW_HEIGHT-(p.pos.y + globals::BOX_HEIGHT)*globals::WINDOW_SCALE,
          p.size.x*globals::WINDOW_SCALE, p.size.y*globals::WINDOW_SCALE]
}