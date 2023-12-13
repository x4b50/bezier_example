use raylib::prelude::*;

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

fn to_color(v: u32) -> Color {
    Color {
        r: (v >> 24 & 0xFF) as u8,
        g: (v >> 16 & 0xFF) as u8,
        b: (v >> 8  & 0xFF) as u8,
        a: (v >> 0  & 0xFF) as u8,
    }
}

fn main() {
    let mut lines: [Option<Line>;2] = [None, None];
    let mut v1: [Option<Vector2>;2] = [None, None];
    let mut v2: [Option<Vector2>;2] = [None, None];
    let mut which = 0;

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Bézier curves")
        .build();
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {which = (which == 0)as usize}

        let ms = d.get_mouse_position();
        if d.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON)
        ||    d.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            v1[which] = Some(ms)
        }
        
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
        ||    d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            v2[which] = Some(ms)
        }

        d.clear_background(Color::BLACK);
        d.draw_text(&format!("{:?}", d.get_fps()), 12, 40, 20, Color::WHITE);
        d.draw_text(&format!("{:?}", which), 12, 20, 20, Color::WHITE);

        for i in 0..v1.len() {
            if let Some(v1) = v1[i] {
                if let Some(v2) = v2[i] {
                    lines[i] = Some(Line { start: v1, end: v2 });
                }
            }
            match &lines[i] {
                Some(l) => {
                    d.draw_line_ex(l.start, l.end, 1., to_color(0x0000ffff));
                } None => {}
            }
        }

        if let Some(l1) = &lines[0] {
            if let Some(l2) = &lines[1] {
                let mut eps = 0.;
                while eps <= 1. {
                    let l = lerp_lines(l1, l2, eps);
                    d.draw_pixel_v(lerp_vec(&l.start, &l.end, eps), to_color(0xff0000ff));
                    eps += 0.001;
                }
                // d.draw_line_ex(l.start, l.end, 1., to_color(0xff00ffff));
            }
        }
    }
}
