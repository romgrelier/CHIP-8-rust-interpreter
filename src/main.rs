mod chip8;
mod screen;

use crate::chip8::interpreter::Interpreter;
use std::env;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = chip8::program::Program::from(String::from(&args[1]));
    let mut interpreter = Interpreter::new();
    interpreter.load_program(&program);


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(&args[1], 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => interpreter.cpu.keys[0] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => interpreter.cpu.keys[1] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => interpreter.cpu.keys[2] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => interpreter.cpu.keys[3] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => interpreter.cpu.keys[4] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => interpreter.cpu.keys[5] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => interpreter.cpu.keys[6] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => interpreter.cpu.keys[7] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => interpreter.cpu.keys[8] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => interpreter.cpu.keys[9] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => interpreter.cpu.keys[10] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => interpreter.cpu.keys[11] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => interpreter.cpu.keys[12] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => interpreter.cpu.keys[13] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => interpreter.cpu.keys[14] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => interpreter.cpu.keys[15] = true,

                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => interpreter.cpu.keys[0] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => interpreter.cpu.keys[1] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => interpreter.cpu.keys[2] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => interpreter.cpu.keys[3] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => interpreter.cpu.keys[4] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => interpreter.cpu.keys[5] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => interpreter.cpu.keys[6] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => interpreter.cpu.keys[7] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => interpreter.cpu.keys[8] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => interpreter.cpu.keys[9] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => interpreter.cpu.keys[10] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => interpreter.cpu.keys[11] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => interpreter.cpu.keys[12] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    ..
                } => interpreter.cpu.keys[13] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => interpreter.cpu.keys[14] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::V),
                    ..
                } => interpreter.cpu.keys[15] = false,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        interpreter.step();

        for y in 0..32 {
            for x in 0..64 {
                if interpreter.cpu.display[(y, x)] {
                    canvas.set_draw_color(Color::BLACK);
                } else {
                    canvas.set_draw_color(Color::WHITE);
                }
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        (x * 10) as i32,
                        (y * 10) as i32,
                        10,
                        10,
                    ))
                    .unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        if interpreter.cpu.delay_timer > 0 {
            interpreter.cpu.delay_timer -= 1;
        }
        if interpreter.cpu.sound_timer > 0 {
            interpreter.cpu.sound_timer -= 1;
        }
    }
}
