
use raylib::prelude::*;

fn main() {
    let _window_height : i32  = 600;
    let _window_width  : i32  = 800;
    
    let _paddle_height : i32 = 150;
    let _paddle_width  : i32 = 25;
    
    let _ball_height : i32 = 20;
    let _ball_width  : i32 = 20;
    
    let _text_size: i32 =  80;
    let _margin   : i32 = 100;
    
    let mut _player1_points: u32 = 0;
    let mut _player2_points: u32 = 0;
    let mut _player1_points_str: String = _player1_points.to_string();
    let mut _player2_points_str: String = _player2_points.to_string();
    
    let mut _paddle1y_pos = (_window_height / 2) - (_paddle_height / 2);
    let mut _paddle1x_pos = 15;
    
    let mut _paddle2y_pos = (_window_height / 2) - (_paddle_height / 2);
    let mut _paddle2x_pos = _window_width - 15 - _paddle_width;
    
    let mut _ballx_pos = (_window_width / 2) - (_ball_width / 2);
    let mut _bally_pos = (_window_height / 2) - (_ball_height / 2);
    
    let mut _ballx_vel = -2;
    let mut _bally_vel = -1;
    
    let (mut _rl, _thread) = raylib::init()
	.size(_window_width, _window_height)
	.title("Hello from Rust!")
	.build();
    
    while !_rl.window_should_close() {
   	let mut _d = _rl.begin_drawing(&_thread);
	
	_d.clear_background(Color::BLACK);
	_d.draw_text(&_player1_points_str, _window_width / 2 - _margin - (_text_size / 2), 10, _text_size, Color::WHITE);
	_d.draw_text(&_player2_points_str, _window_width / 2 + _margin + (_text_size / 2), 10, _text_size, Color::WHITE);
	
	_d.draw_rectangle(_paddle1x_pos, _paddle1y_pos, _paddle_width, _paddle_height, Color::WHITE);
	_d.draw_rectangle(_paddle2x_pos, _paddle2y_pos, _paddle_width, _paddle_height, Color::WHITE);
	
	_d.draw_rectangle(_ballx_pos, _bally_pos, _ball_width, _ball_height, Color::WHITE);
	
	if _d.is_key_down(KeyboardKey::KEY_W) {
	    _paddle1y_pos -= 2;
	}
	else if _d.is_key_down(KeyboardKey::KEY_S) {
	    _paddle1y_pos += 2;
	}
	_paddle2y_pos = _bally_pos - (_paddle_height / 2) + (_ball_height / 2);

	if  _ballx_pos < _paddle1x_pos + _paddle_width  &&
	    _ballx_pos + _ball_width > _paddle1x_pos    &&
	    _bally_pos < _paddle1y_pos + _paddle_height &&
	    _bally_pos + _ball_height > _paddle1y_pos 
	    ||
	    _ballx_pos < _paddle2x_pos + _paddle_width  &&
	    _ballx_pos + _ball_width > _paddle2x_pos    &&
	    _bally_pos < _paddle2y_pos + _paddle_height &&
	    _bally_pos + _ball_height > _paddle2y_pos 
	{
		_ballx_vel = -_ballx_vel;
		// _ballx_vel *=  1;
		// _bally_vel *=  1;
	}
	else if _bally_pos < 0 || 
	    _bally_pos + _ball_height > _window_height || 
	    _bally_pos + _ball_height > _window_height 
	{
	    _bally_vel = -_bally_vel;
	}
	
	_ballx_pos += _ballx_vel;
	_bally_pos += _bally_vel;
	
    }
}
