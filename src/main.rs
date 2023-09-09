extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut y_position = 10;
    let mut x_position = 10;
    let mut direction_current = "up";
    
    let window = video_subsystem.window("Snake", 400, 400)
        .position_centered()
        .build()
        .unwrap();
    
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    

    
    
    
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'stop: loop {
        let background = Rect::new(0, 0, 400, 400);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(background);

        let snake = Rect::new(x_position * 20, y_position * 20, 20, 20);
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(snake);    
        canvas.present();

      ::std::thread::sleep(Duration::from_millis(200));
      for event in event_pump.poll_iter() {
        println!("{:?}", event);
        match event {
            Event::Quit{timestamp} => {
                println!("Quit detetcted");
                break 'stop;
            }
            Event::KeyUp{timestamp, keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                println!("Up detected");
                direction_current = "up";
            }
            Event::KeyDown{timestamp, keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                println!("Down detected");
                direction_current = "down";
            }
            Event::KeyUp{timestamp, keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                println!("Right detected");
                direction_current = "right";
            }
            Event::KeyUp{timestamp, keycode: Some(sdl2::keyboard::Keycode::Left), ..} => {
                println!("Left detected");
                direction_current = "left";             
            }
            _ => {
                println!("something else");
            }
        }
      }

      if direction_current == "up" {
        y_position = y_position - 1;
        println!("{}", y_position)
      }
      else if direction_current == "down" {
        y_position = y_position + 1;
        println!("{}", y_position)
      }
      else if direction_current == "left" {
        x_position = x_position - 1;
            println!("{}", x_position)
      }
      else if direction_current == "right" {
        x_position = x_position + 1;
        println!("{}", x_position)
      }

      canvas.clear()
    }

    
    // }
}