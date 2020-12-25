extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
use piston_window::{Event, Input, Button, ButtonState, Key, Loop, UpdateArgs, RenderArgs};
use piston_window::{clear, rectangle};

// FORCES
const GRAVITY: f64 = 9.81;  // force of gravity always applied downwards
const JUMP_FORCE: f64 = 50.0;  // force applied while jumping
const DRIFT_FORCE: f64 = 5.0;  // force applied by horizontal movement
const GND_DRAG_FORCE: f64 = 0.5;  // friction force applied opposing velocity while grounded
const AIR_DRAG_FORCE: f64 = 0.1;  // friction force applied opposing velocity in the air


// IMPULSES
const JUMP_DURATION: i32 = 12;  // max duration of jump impulse in frames
const DRIFT_DURATION: i32 = 40;  // max duration of run impulse in frames

// SPEEDS
const MIN_XSPEED: f64 = 0.01;  // x-speeds less than this are assumed to not be moving
const MAX_XSPEED: f64 = 2.0;  // x-speeds greater than this are clamped
const MIN_YSPEED: f64 = 0.01;  // y-speeds less than this are assumed to not be moving
const MAX_YSPEED: f64 = 5.0;  // y-speeds greater than this are clamped

// COORDINATES
const WINDOW_WIDTH: f64 = 640.0;  // width of game window in pixels
const WINDOW_HEIGHT: f64 = 480.0;  // height of game window in pixels
const WINDOW_SCALE: f64 = 100.0;  // convert pixel coords to meters
const WIDTH: f64 = WINDOW_WIDTH / WINDOW_SCALE;  // width of game world in meters
const HEIGHT: f64 = WINDOW_HEIGHT / WINDOW_SCALE;  // height of game world in meters
const BOX_WIDTH: f64 = 0.2;  // width of the player box in meters
const BOX_HEIGHT: f64 = 0.3;  // height of the player box in meters

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
    WindowSettings::new("Jump Game", [WINDOW_WIDTH, WINDOW_HEIGHT])
    .exit_on_esc(true).build().unwrap();

  let p1 = Player {
  	pos: Vector{x:2.0*BOX_WIDTH, y:2.0*BOX_HEIGHT+0.1},
  	vel: Vector{x:0.0, y:0.0},
  	acc: Vector{x:0.0, y:0.0},
  	size: Vector{x:2.0*BOX_WIDTH, y:2.0*BOX_HEIGHT},
  	color: [1.0, 0.0, 0.0, 1.0],
  	keys: KeyState{left: false, right: false, jump: false},
    jump_time: 0,
    drift_time: 0,
    grounded: false
  };

  let p2 = Player {
  	pos: Vector{x:WIDTH - 2.0*BOX_WIDTH, y:2.0*BOX_HEIGHT+0.1},
  	vel: Vector{x:0.0, y:0.0},
  	acc: Vector{x:0.0, y:0.0},
  	size: Vector{x:2.0*BOX_WIDTH, y:2.0*BOX_HEIGHT},
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
  update_player(&mut game.p2, update_args.dt);
}

fn update_player(p: &mut Player, dt: f64) {
	// determine accelerations based on states
	p.acc.y = -GRAVITY;
	if p.keys.jump {
    if p.jump_time < JUMP_DURATION {
      p.acc.y = JUMP_FORCE;
      p.jump_time += 1;
    }
  } else if p.grounded {
    p.jump_time = 0;
  } else {
    p.jump_time = JUMP_DURATION;
  }

	if p.keys.left { 
    if p.drift_time < DRIFT_DURATION {
      p.acc.x = -DRIFT_FORCE;
      p.drift_time += 1;
    }
  } else if p.keys.right {
    if p.drift_time < DRIFT_DURATION {
      p.acc.x = DRIFT_FORCE;
      p.drift_time += 1;
    }
  } else {
    if p.vel.x >= MIN_XSPEED {
      if p.grounded {
        p.acc.x -= GND_DRAG_FORCE;
      } else {
        p.acc.x -= AIR_DRAG_FORCE;
      }
      if p.vel.x + p.acc.x < MIN_XSPEED {
        p.vel.x = 0.0;
        p.acc.x = 0.0;
      }
    }
    if p.vel.x <= -MIN_XSPEED {
      if p.grounded {
        p.acc.x += GND_DRAG_FORCE;
      } else {
        p.acc.x += AIR_DRAG_FORCE;
      }
      if p.vel.x + p.acc.x > -MIN_XSPEED {
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
  if p.pos.y <= BOX_HEIGHT {  // floor
    p.pos.y = BOX_HEIGHT + MIN_YSPEED*dt;
    p.vel.y = 0.0;
  } else if p.pos.y >= HEIGHT {  // ceiling
    p.pos.y = HEIGHT - MIN_YSPEED*dt;
    p.vel.y = 0.0;
  }
  if p.pos.x <= 0.0 {  // left
    p.pos.x = 0.0 + MIN_XSPEED*dt;
    p.vel.x = 0.0;
  } else if p.pos.x >= WIDTH {  // right
    p.pos.x = WIDTH - MIN_XSPEED*dt;
    p.vel.x = 0.0;
  }

  p.grounded = p.pos.y <= BOX_HEIGHT + MIN_YSPEED*dt;
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
  return [(p.pos.x - BOX_WIDTH)*WINDOW_SCALE,
          WINDOW_HEIGHT-(p.pos.y + BOX_HEIGHT)*WINDOW_SCALE,
          p.size.x*WINDOW_SCALE, p.size.y*WINDOW_SCALE]
}