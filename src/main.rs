extern crate piston_window;

use piston_window::*;


struct Vector {
	x: f64,
	y: f64
}

struct Player {
	pos: Vector,
	vel: Vector,
	acc: Vector,
	size: Vector,
	color: [f32; 4]
}


fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut p = Player {
    	pos: Vector{x:0.0, y:0.0},
    	vel: Vector{x:0.0, y:0.0},
    	acc: Vector{x:0.0, y:0.0},
    	size: Vector{x:100.0, y:100.0},
    	color: [1.0, 0.0, 0.0, 1.0]
    };
    
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(p.color, // red
                      [p.pos.x, p.pos.y, p.size.x, p.size.y],
                      context.transform,
                      graphics);
        });
        
        if let Some(Button::Keyboard(Key::Left)) = event.press_args() {
            p.pos.x -= 5.0;
        }
        if let Some(Button::Keyboard(Key::Right)) = event.press_args() {
            p.pos.x += 5.0;
        }
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            p.pos.y -= 5.0;
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            p.pos.y += 5.0;
        }
    }
}

// fn update_player(p: &Player) {
// 	p.x
// }
