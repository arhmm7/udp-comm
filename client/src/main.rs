use std::net::UdpSocket;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
const MAX_MESSAGES: usize = 10;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_nonblocking(true).unwrap();
    let server_addr = "127.0.0.1:4000";

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Text Client")
        .vsync()
        .log_level(TraceLogLevel::LOG_NONE)
        .build();

    let mut input_text = String::new();
    let mut buffer = [0u8; 1024];
    let mut messages: Vec<String> = Vec::new();

    while !rl.window_should_close() {
        while let Some(c) = rl.get_char_pressed() {
            if !c.is_control() {
                input_text.push(c);
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            input_text.pop();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) && !input_text.is_empty() {
            let _ = socket.send_to(input_text.as_bytes(), server_addr);
            input_text.clear();
        }

        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, _)) => {
                    let msg = String::from_utf8_lossy(&buffer[..size]).to_string();
                    messages.push(msg);
                    if messages.len() > MAX_MESSAGES {
                        messages.remove(0);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        let fps = rl.get_fps().to_string();
        let time = SystemTime::now();
        let datetime: DateTime<Local> = time.into();
        let formatted_time = datetime.format("%I:%M:%S").to_string();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_text(&fps, 12, 12, 20, Color::WHITE);
        d.draw_text(&formatted_time, 12, 34, 20, Color::WHITE);

        let mut y = 70;
        for msg in &messages {
            d.draw_text(msg, 12, y, 20, Color::LIME);
            y += 22;
        }

        d.draw_rectangle_lines(10, 440, 620, 30, Color::WHITE);
        d.draw_text(&input_text, 16, 445, 20, Color::WHITE);
    }
}
