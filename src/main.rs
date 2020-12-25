extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
use piston_window::{Event, Input, Button, ButtonState, Key, Loop, UpdateArgs, RenderArgs};
use piston_window::{clear, rectangle};

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
	keys: KeyState
}

struct Game {
	p1: Player,
	p2: Player
}

fn main() {
  let mut window: PistonWindow =
    WindowSettings::new("Jump Game", [640, 480])
    .exit_on_esc(true).build().unwrap();

  let p1 = Player {
  	pos: Vector{x:0.0, y:0.0},
  	vel: Vector{x:0.0, y:0.0},
  	acc: Vector{x:0.0, y:0.0},
  	size: Vector{x:100.0, y:100.0},
  	color: [1.0, 0.0, 0.0, 1.0],
  	keys: KeyState{left: false, right: false, jump: false}
  };

  let p2 = Player {
  	pos: Vector{x:200.0, y:0.0},
  	vel: Vector{x:0.0, y:0.0},
  	acc: Vector{x:0.0, y:0.0},
  	size: Vector{x:100.0, y:100.0},
  	color: [1.0, 0.0, 0.0, 1.0],
  	keys: KeyState{left: false, right: false, jump: false}
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
	let p1 = &game.p1;
	let p2 = &game.p2;

	// p1.acc.x = GRAVITY + applied.x
	// p1.vel.x += p1.acc.x;
	// p1.pos.x += p1.vel.x;
}

fn render(game: &Game, window: &mut PistonWindow, event: Event, render_args: &RenderArgs){
	let p1 = &game.p1;
	let p2 = &game.p2;

	let w = render_args.window_size[0];
	let h = render_args.window_size[1];

	window.draw_2d(&event, |c, g, _d| {
    clear([1.0; 4], g);	 // clear the screen 
    rectangle([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, w, h], c.transform, g);	// draw the background

    rectangle(p1.color, 
    	[p1.pos.x, p1.pos.y, p1.size.x, p1.size.y],
    	c.transform, g);  // draw player 1

    rectangle(p2.color, 
    	[p2.pos.x, p2.pos.y, p2.size.x, p2.size.y],
    	c.transform, g);  // draw player 2
  });
}
