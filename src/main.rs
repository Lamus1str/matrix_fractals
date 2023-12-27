extern crate sdl2;

use std::io::stdin;
use std::str::FromStr;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn draw_f(x: i32, y: i32, scale: [i32; 2], f: &Vec<Vec<i32>>, canv: &mut sdl2::render::WindowCanvas){
    let (mut width, mut height) = (scale[0], scale[1]);
    let (mut _x, mut _y) = (0, 0);
    let y_size = f.len() as i32;
    //println!("y = {y_size}");
    for line in f{
        let x_size = line.len() as i32;
        //println!("{x_size} line = {:?}", line);
        _x = 0;
        for flag in line{
            if *flag == 1 {
                if scale[1] <= 1 {
                    let _ = canv.draw_point(Point::new(x + (_x as i32), y + (_y as i32)));
                }
                else {
                    draw_f(x + ((_x as i32) * scale[1]),
                           y + ((_y as i32) * scale[0]),
                           [scale[0]/y_size, scale[1]/x_size],
                           f, canv);
                }
            }
            _x += 1;
        }
        _y += 1;
    }
}

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("FFFFF", 512, 512)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut input: String = String::new();
    match stdin().read_line(&mut input){
        Ok(_) =>{
            let mut input_split = input.split_whitespace();
            if let (Some(inp_0), Some(inp_1)) = (input_split.next(), input_split.next()){
                if let (Ok(a), Ok(b)) =
                       (usize::from_str(inp_0), usize::from_str(inp_1))
                {
                    let mut f: Vec<Vec<i32>> = vec![vec![0; a]; b];
                    for line in &mut f{
                        let mut line_inp: String = String::new();
                        if let Ok(_) = stdin().read_line(&mut line_inp){
                            let mut i = 0;
                            for val in line_inp.split_whitespace(){
                                if let Ok(1) = u8::from_str(val){
                                    if i < a{
                                        line[i] = 1;

                                    }
                                    else { break; }
                                }
                                i += 1;
                            }
                        }
                    }
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                    draw_f(0, 0, [64, 64], &f, &mut canvas);
                }
            }
        }
        _ => {},
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {

                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
