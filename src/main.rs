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

pub struct Vector {
	x: f64,
	y: f64
}

pub struct KeyState {
	left: bool,
	right: bool,
	jump: bool
}

pub struct Player {
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

pub struct Game {
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

  let p2 = Player {
  	pos: Vector{x:globals::WIDTH - 2.0*globals::BOX_WIDTH, y:2.0*globals::BOX_HEIGHT+0.1},
  	vel: Vector{x:0.0, y:0.0},
  	acc: Vector{x:0.0, y:0.0},
  	size: Vector{x:2.0*globals::BOX_WIDTH, y:2.0*globals::BOX_HEIGHT},
  	color: [0.0, 1.0, 0.0, 1.0],
  	keys: KeyState{left: false, right: false, jump: false},
    jump_time: 0,
    drift_time: 0,
    grounded: false
  };
  
  let mut game = Game {
  	p1: p1,
  	p2: p2
  };

  // connect to web socket
  println!("Connecting to {}", globals::CONNECTION);

  let client = ClientBuilder::new(globals::CONNECTION)
    .unwrap()
    .add_protocol("rust-websocket")
    .connect_insecure()
    .unwrap();

  println!("Successfully connected");
  // TODO
  // when connected, add the new player to the game state
  // broadcast current game state to the new player that joined
  // anytime update function is called, need to broadcast new states

  while let Some(event) = window.next() {
    match event {
      Event::Input(input_args, _timestamp) => { process_keys(&mut game, &input_args); },
      Event::Loop(loop_args) => {
      	match loop_args {
      		Loop::Update(update_args) => { update(&mut game, &update_args); },
      		Loop::Render(render_args) => { render(&game, &mut window, event, &render_args); },
      		_ => {}
      	}},
      _ => {}
    }
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
}

fn render(game: &Game, window: &mut PistonWindow, event: Event, render_args: &RenderArgs){
	let p1 = &game.p1;
	let p2 = &game.p2;

	window.draw_2d(&event, |c, g, _d| {
    clear([0.0, 0.0, 0.0, 1.0], g);	 // clear the screen

    rectangle(p1.color,	to_window_coords(&p1), c.transform, g);  // draw player 1
    rectangle(p2.color, to_window_coords(&p2), c.transform, g);  // draw player 2
  });
}

fn to_window_coords(p: &Player) -> [f64; 4] {
  return [(p.pos.x - globals::BOX_WIDTH)*globals::WINDOW_SCALE,
          globals::WINDOW_HEIGHT-(p.pos.y + globals::BOX_HEIGHT)*globals::WINDOW_SCALE,
          p.size.x*globals::WINDOW_SCALE, p.size.y*globals::WINDOW_SCALE]
}