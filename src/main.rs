
use raylib::prelude::*;

fn main() {
    let _window_height : i32  = 600;
    let _window_width  : i32  = 800;
    
    let _paddle_height : i32 = 150;
    let _paddle_width  : i32 = 25;
    
    let _ball_height : i32 = 25;
    let _ball_width  : i32 = 25;
    
    let _text_size: i32 =  80;
    let _margin   : i32 = 100;
    
    let mut _player1_points: u32 = 0;
    let mut _player2_points: u32 = 0;
    let mut _player1_points_str: String = _player1_points.to_string();
    let mut _player2_points_str: String = _player2_points.to_string();
    
    let (mut _rl, _thread) = raylib::init()
	.size(_window_width, _window_height)
	.title("Hello from Rust!")
	.build();
    
    while !_rl.window_should_close() {
   	let mut _d = _rl.begin_drawing(&_thread);
	
	_d.clear_background(Color::BLACK);
	_d.draw_text(&_player1_points_str, _window_width / 2 - _margin - (_text_size / 2), 10, _text_size, Color::WHITE);
	_d.draw_text(&_player2_points_str, _window_width / 2 + _margin + (_text_size / 2), 10, _text_size, Color::WHITE);
	
	_d.draw_rectangle(15, (_window_height / 2) - (_paddle_height / 2), _paddle_width, _paddle_height, Color::WHITE);
	_d.draw_rectangle(_window_width - 15 - _paddle_width, (_window_height / 2) - (_paddle_height / 2), _paddle_width, _paddle_height, Color::WHITE);
	
	_d.draw_rectangle((_window_width / 2) - (_ball_width / 2), (_window_height / 2)  - (_ball_width / 2), _ball_width, _ball_height, Color::WHITE);
    }
}
