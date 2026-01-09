use std::collections::HashMap;
use std::net::UdpSocket;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
const SPEED: i32 = 2;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_nonblocking(true).unwrap();
    let server_addr = "127.0.0.1:4000";

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("UDP Game Client")
        .vsync()
        .build();

    let mut buffer = [0u8; 1024];
    let mut players: HashMap<String, (i32, i32)> = HashMap::new();

    while !rl.window_should_close() {
        let mut dx = 0;
        let mut dy = 0;

        if rl.is_key_down(KeyboardKey::KEY_W) {
            dy -= SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            dy += SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            dx -= SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            dx += SPEED;
        }

        if dx != 0 || dy != 0 {
            let msg = format!("MOVE {} {}", dx, dy);
            let _ = socket.send_to(msg.as_bytes(), server_addr);
        }

        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, _)) => {
                    let data = String::from_utf8_lossy(&buffer[..size]);
                    for line in data.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() == 3 {
                            let id = parts[0].to_string();
                            let x = parts[1].parse().unwrap_or(0);
                            let y = parts[2].parse().unwrap_or(0);
                            players.insert(id, (x, y));
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for (_id, (x, y)) in &players {
            let sx = SCREEN_WIDTH / 2 + x;
            let sy = SCREEN_HEIGHT / 2 + y;
            d.draw_circle(sx, sy,20.0,Color::WHITE);
        }
    }
}
