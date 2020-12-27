extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
use piston_window::{Event, Input, Button, ButtonState, Key, Loop, UpdateArgs, RenderArgs};
use piston_window::{clear, rectangle};

extern crate websocket;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

use websocket::client::ClientBuilder;
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
	p2: Player
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

  while let Some(event) = window.next() {
    match event {
      Event::Input(input_args, _timestamp) => { send_key(&mut game, &input_args); },
      Event::Loop(loop_args) => {
        match loop_args {
          // Loop::Update(update_args) => { update(&mut game, &update_args); },
          Loop::Render(render_args) => { render(&game, &mut window, event, &render_args); },
          _ => {}
        }},
      _ => {}
    }
  }
}

fn send_key(game: &mut Game, input: &Input) {
  // send the keys to websocket
}

fn render(game: &Game, window: &mut PistonWindow, event: Event, render_args: &RenderArgs){
	let p1 = &game.p1;
	let p2 = &game.p2;

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