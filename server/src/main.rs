use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();
    println!("Server listening on {}", socket.local_addr().unwrap());

    let mut buffer = [0u8; 1024];
    let mut players: HashMap<SocketAddr, (i32, i32)> = HashMap::new();

    loop {
        let (size, source) = socket.recv_from(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..size]);

        let entry = players.entry(source).or_insert((0, 0));

        let parts: Vec<&str> = message.trim().split_whitespace().collect();

        if parts.len() == 3 && parts[0] == "MOVE" {
            let dx: i32 = parts[1].parse().unwrap_or(0);
            let dy: i32 = parts[2].parse().unwrap_or(0);
            entry.0 += dx;
            entry.1 += dy;
        }

        let mut state = String::new();
        for (addr, (x, y)) in &players {
            state.push_str(&format!("{} {} {}\n", addr, x, y));
        }

        for client in players.keys() {
            socket.send_to(state.as_bytes(), client).unwrap();
        }
    }
}
