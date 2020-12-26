pub static IP: &'static str = "127.0.0.1:2794";
pub static CONNECTION: &'static str = "ws://127.0.0.1:2794";

// FORCES
pub static GRAVITY: f64 = 9.81;  // force of gravity always applied downwards
pub static JUMP_FORCE: f64 = 50.0;  // force applied while jumping
pub static DRIFT_FORCE: f64 = 5.0;  // force applied by horizontal movement
pub static GND_DRAG_FORCE: f64 = 0.5;  // friction force applied opposing velocity while grounded
pub static AIR_DRAG_FORCE: f64 = 0.1;  // friction force applied opposing velocity in the air

// IMPULSES
pub static JUMP_DURATION: i32 = 12;  // max duration of jump impulse in frames
pub static DRIFT_DURATION: i32 = 40;  // max duration of run impulse in frames

// SPEEDS
pub static MIN_XSPEED: f64 = 0.01;  // x-speeds less than this are assumed to not be moving
pub static MAX_XSPEED: f64 = 2.0;  // x-speeds greater than this are clamped
pub static MIN_YSPEED: f64 = 0.01;  // y-speeds less than this are assumed to not be moving
pub static MAX_YSPEED: f64 = 5.0;  // y-speeds greater than this are clamped

// COORDINATES
pub static WINDOW_WIDTH: f64 = 640.0;  // width of game window in pixels
pub static WINDOW_HEIGHT: f64 = 480.0;  // height of game window in pixels
pub static WINDOW_SCALE: f64 = 100.0;  // convert pixel coords to meters
pub static WIDTH: f64 = WINDOW_WIDTH / WINDOW_SCALE;  // width of game world in meters
pub static HEIGHT: f64 = WINDOW_HEIGHT / WINDOW_SCALE;  // height of game world in meters
pub static BOX_WIDTH: f64 = 0.2;  // width of the player box in meters
pub static BOX_HEIGHT: f64 = 0.3;  // height of the player box in meters