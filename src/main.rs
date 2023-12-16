use std::{thread::{sleep, self}, time::{Duration, Instant}};

use raylib::prelude::*;

#[derive(Debug)]
struct Line {
    start: Vector2,
    end:   Vector2,
}

fn lerp_vec(v1: &Vector2, v2: &Vector2, s: f32) -> Vector2 {
    Vector2 {
        x: v1.x + (v2.x-v1.x)*s,
        y: v1.y + (v2.y-v1.y)*s,
    }
}

fn lerp_lines(l1: &Line, l2: &Line, s: f32) -> Line {
    Line {
        start: lerp_vec(&l1.start, &l1.end, s),
        end: lerp_vec(&l2.start, &l2.end, s)
    }
}

fn bezier(lines: &[&Line], s: f32, show: bool, d: &mut RaylibDrawHandle) -> Line {
    if lines.len() < 2 {panic!("Something went wrong, too little lines")}
    if lines.len() == 2 {
        if show { d.draw_line_v(
                lerp_vec(&lines[0].start, &lines[0].end, s),
                lerp_vec(&lines[1].start, &lines[1].end, s),
                to_color(0x00ff0066));
        }
        return lerp_lines(&lines[0], &lines[1], s);
    } else {
        let l1 = bezier(&lines[..lines.len()-1], s, show, d);
        let l2 = bezier(&lines[1..], s, show, d);
        if show { d.draw_line_v(
                lerp_vec(&l1.start, &l1.end, s),
                lerp_vec(&l2.start, &l2.end, s),
                to_color(0xbbbbbb66));
        }
        return lerp_lines(&l1, &l2, s);
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
    let mut lines: Vec<Option<Line>> = vec![None, None];
    let mut v1: Vec<Option<Vector2>> = vec![None, None];
    let mut v2: Vec<Option<Vector2>> = vec![None, None];
    let mut which = 0;
    let mut intermidiete = true;
    let mut animate = false;
    let mut eps = 200.;

    let start = Instant::now();
    let frame_time = Duration::new(0, 1000000000u32/125);

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Bézier curves")
        .build();
    // rl.set_exit_key(Some(KeyboardKey::KEY_Q));
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let frame_start = Instant::now();
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if which+1 < lines.len() {which+=1}
            else {which=0}
        }

        if d.is_key_down(KeyboardKey::KEY_MINUS) {
            if eps > 100. {eps -= 10.}
            eps -= 1.; sleep(Duration::from_millis(25));
        }
        if d.is_key_down(KeyboardKey::KEY_EQUAL) {
            if eps > 100. {eps += 10.}
            eps += 1.; sleep(Duration::from_millis(25));
        }
        if eps < 0. {eps = 0.}
        if eps > 1000. {eps = 1000.}

        if d.is_key_pressed(KeyboardKey::KEY_I) {animate = false; intermidiete = !intermidiete;}
        if d.is_key_pressed(KeyboardKey::KEY_A) {animate = !animate;}

        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            which = lines.len();
            lines.push(None);
            v1.push(None);
            v2.push(None);
        }

        let ms = d.get_mouse_position();
        if d.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON)
        ||    d.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            v1[which] = Some(ms)
        }
        
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
        ||    d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            v2[which] = Some(ms)
        }

        if d.is_key_pressed(KeyboardKey::KEY_C) {
            lines = vec![None, None];
            v1 = vec![None, None];
            v2 = vec![None, None];
            which = 0;
        }

        d.clear_background(Color::BLACK);
        d.draw_text(&format!("{:?}", which+1), 12, 12, 20, Color::WHITE);
        d.draw_text(&format!("{:?}", d.get_fps()), 12, 30, 20, Color::WHITE);

        for i in 0..lines.len() {
            if let Some(v1) = v1[i] {
                if let Some(v2) = v2[i] {
                    lines[i] = Some(Line { start: v1, end: v2 });
                }
            }
            match &lines[i] {
                Some(l) => {
                    d.draw_line_ex(l.start, l.end, 1., Color {
                        r: (255 * i+1/lines.len())as u8,
                        g: 0,
                        b: 255,
                        a: 255,
                    });
                } None => {}
            }
        }

        if let Some(_) = &lines[0] {
            if let Some(_) = &lines[1] {
                let mut s = 0.;
                let mut ls = vec![];
                for line in &lines {
                    match line {
                        Some(l) => {ls.push(l)}
                        None => {}
                    }
                }
                for i in 0..ls.len()-1 {
                    d.draw_line_v(ls[i].end, ls[i+1].start, to_color(0x00ff0066))
                }
                if animate {
                    let s = start.elapsed().as_millis();
                    let s = ((s as f32/500.).sin() +1.)/2.;
                    let l = bezier(&ls[..], s, true, &mut d);
                    d.draw_circle_v(lerp_vec(&l.start, &l.end, s), eps/15., to_color(0xff0000ff));
                } else {
                    while s <= 1. {
                        let l = bezier(&ls[..], s, intermidiete, &mut d);
                        d.draw_pixel_v(lerp_vec(&l.start, &l.end, s), to_color(0xff0000ff));
                        s += 1./eps;
                    }
                }
            }
        }

        let frame_d = frame_start.elapsed();
        if frame_d < frame_time {
            thread::sleep(frame_time - frame_d);
        }
    }
}
