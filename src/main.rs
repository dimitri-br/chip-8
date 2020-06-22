pub mod cpu;

use std::process::Command;

use cpu::{emulate_cycle, load_rom, load};
use std::{thread, time};
use std::io::{stdout,Write};

extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;



const WIDTH : u32 = 640;
const HEIGHT : u32 = 320;

fn main(){
    //calculate scale
    let scale_x = (WIDTH / 64) as u16;
    let scale_y = (HEIGHT / 32) as u16;

    let mut paused = false;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("chip-8", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    
    let mut cpu = load();
    
    cpu = load_rom(cpu);
    

    'running: loop {
        if !paused{
            canvas.set_draw_color(Color::RGB(0,0,0));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running;
                    },
                    Event::KeyDown { keycode, .. } => {
                        match keycode  {
                            Some(Keycode::Escape) => break 'running,
                            //handle user input. 1 is on, 0 is off. Change key addresses per key
                            Some(Keycode::Num1) => cpu.key[0x1] =  1,
                            Some(Keycode::Num2) => cpu.key[0x2] =  1,
                            Some(Keycode::Num3) => cpu.key[0x3] =  1,
                            Some(Keycode::Num4) => cpu.key[0xC] =  1,

                            Some(Keycode::Q) => cpu.key[0x4] =  1,
                            Some(Keycode::W) => cpu.key[0x5] =  1,
                            Some(Keycode::E) => cpu.key[0x6] =  1,
                            Some(Keycode::R) => cpu.key[0xD] =  1,

                            Some(Keycode::A) => cpu.key[0x7] =  1,
                            Some(Keycode::S) => cpu.key[0x8] =  1,
                            Some(Keycode::D) => cpu.key[0x9] =  1,
                            Some(Keycode::F) => cpu.key[0xE] =  1,

                            Some(Keycode::Z) => cpu.key[0xA] =  1,
                            Some(Keycode::X) => cpu.key[0x0] =  1,
                            Some(Keycode::C) => cpu.key[0xB] =  1,
                            Some(Keycode::V) => cpu.key[0xF] =  1,
                            //Some(Keycode::P) => paused = !paused,
                            _ => {}
                            
                            

                        }
                        
                    },
                    Event::KeyUp { keycode, .. } =>{
                        match keycode{
                            Some(Keycode::Num1) => cpu.key[0x1] =  0,
                            Some(Keycode::Num2) => cpu.key[0x2] =  0,
                            Some(Keycode::Num3) => cpu.key[0x3] =  0,
                            Some(Keycode::Num4) => cpu.key[0xC] =  0,

                            Some(Keycode::Q) => cpu.key[0x4] =  0,
                            Some(Keycode::W) => cpu.key[0x5] =  0,
                            Some(Keycode::E) => cpu.key[0x6] =  0,
                            Some(Keycode::R) => cpu.key[0xD] =  0,

                            Some(Keycode::A) => cpu.key[0x7] =  0,
                            Some(Keycode::S) => cpu.key[0x8] =  0,
                            Some(Keycode::D) => cpu.key[0x9] =  0,
                            Some(Keycode::F) => cpu.key[0xE] =  0,

                            Some(Keycode::Z) => cpu.key[0xA] =  0,
                            Some(Keycode::X) => cpu.key[0x0] =  0,
                            Some(Keycode::C) => cpu.key[0xB] =  0,
                            Some(Keycode::V) => cpu.key[0xF] =  0,
                            _ => {}
                        }

                    },
                    
                    _ => {}
                }
            }
            
            // The rest of the game loop goes here...
            cpu = emulate_cycle(cpu);

            //draw
            canvas.set_draw_color(Color::RGB(255, 255, 255));




            if cpu.draw{
                for y in 0..32{
                    for x in 0..64{
                        if cpu.vram[(y as usize*64) + x as usize] == 0{
                                //off
                        }else{
                            canvas.fill_rect(Rect::new((x * scale_x).into(), (y * scale_y).into(), (1 * scale_x).into(), (1 * scale_y).into())).unwrap();    //on
                        }
                    }
                
                }
                //cpu.draw = false;
            }

            canvas.present();



            //::std::thread::sleep(Duration::from_secs(1));
        }
    }
    
    
}



/*
//terminal output
fn draw(cpu : CPU) -> CPU{
    let timer = time::Duration::from_millis(0);
    
    Command::new("cmd")
            .args(&["/C", "cls"])
            .output()
            .expect("failed to execute process");
    let mut buffer = Vec::<char>::new();
    for y in 0..32{
        for x in 0..64{
            if cpu.get_gfx((y*64) + x).expect("error loading 778") == 0{
                buffer.push(32 as char);
            }else{
                buffer.push(35 as char);
            }
        }
        
        buffer.push(10 as char);
    }

    buffer.push(10 as char);
    thread::sleep(timer);
    let s : String = buffer.into_iter().collect();
    println!("{}",s);
    cpu 
}*/