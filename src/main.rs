use std::{thread::sleep, time::Duration};

use raylib::prelude::*;

struct Line {
    start: Vector2,
    end: Vector2,
}

fn line(x1:f32, y1:f32, x2:f32, y2:f32) -> Line {
    Line {
        start: Vector2 { x: x1, y: y1 },
        end:   Vector2 { x: x2, y: y2 },
    }
}

fn to_color(v: u32) -> Color {
    Color {
        r: (v >> 24 & 0xFF) as u8,
        g: (v >> 16 & 0xFF) as u8,
        b: (v >> 8  & 0xFF) as u8,
        a: (v >> 0  & 0xFF) as u8,
    }
}

fn main() {
    let mut line = Line {
        start: Vector2 {x: 10., y: 10.},
        end: Vector2 {x: 100., y: 250.},
    };

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Bézier curves")
        .build();
     
    while !rl.window_should_close() {
        let key = rl.get_key_pressed();
        let space = rl.is_key_down(KeyboardKey::KEY_SPACE);
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);
        d.draw_text(&format!("{:?}", match key {
            Some(k) => {k}
            None => {KeyboardKey::KEY_NULL}
        }), 12, 12, 20, Color::BLACK);
        d.draw_text(&format!("{:?}", space), 12, 24, 20, Color::BLACK);
        d.draw_text(&format!("{:?}", d.get_fps()), 12, 40, 20, Color::BLACK);

        d.draw_line_ex(line.start, line.end, 1., to_color(0x0000ffff));
        // sleep(Duration::from_millis(100));
        // let ms = d.get_mouse_position();
    }
}
