extern crate piston_window;

use piston_window::*;

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
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut p1 = Player {
    	pos: Vector{x:0.0, y:0.0},
    	vel: Vector{x:0.0, y:0.0},
    	acc: Vector{x:0.0, y:0.0},
    	size: Vector{x:100.0, y:100.0},
    	color: [1.0, 0.0, 0.0, 1.0],
    	keys: KeyState{left: false, right: false, jump: false}
    };

    let mut p2 = Player {
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
	        Event::Input(input) => { process_keys(&game, &input); },
	        Event::Update(update_args) => { update(&game); }
	        Event::Render(render_args) => { render(&game, &window, &event); }
	    }
        
    }
}

fn process_keys(game: &Game, input: &Input) {
	match input {
        Input::Press(butpress) => { 
        	match butpress {
	            Button::Keyboard(Key::Space) => { game.p1.keys.jump = true; }
	            Button::Keyboard(Key::Left) => { game.p1.keys.left = true; }
	            Button::Keyboard(Key::Right) => { game.p1.keys.right = true; }
	            _ => {}
        }}
        Input::Release(butrelease) => {
        	match butrelease {
	            Button::Keyboard(Key::Space) => { game.p1.keys.jump = false; }
	            Button::Keyboard(Key::Left) => { game.p1.keys.left = false; }
	            Button::Keyboard(Key::Right) => { game.p1.keys.right = false; }
	            _ => {}
        }}
    }
}

fn update(game: &Game) {
	game.p1.pos.x += 5.0;
	game.p2.pos.x -= 5.0;
}

fn render(game: &Game, window: &PistonWindow, event: &dyn GenericEvent) {
    window.draw_2d(&event, |context, graphics, _device| {
        clear([1.0; 4], graphics);
        rectangle(game.p1.color, // red
                  [game.p1.pos.x, game.p1.pos.y, game.p1.size.x, game.p1.size.y],
                  context.transform,
                  graphics);
    });
}