use std::net::{SocketAddr, UdpSocket};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();
    println!("Server listening on {}", socket.local_addr().unwrap());

    let mut buffer = [0u8; 1024];
    let mut clients: Vec<SocketAddr> = Vec::new();

    loop {
        let (size, source) = socket.recv_from(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..size]);

        if !clients.contains(&source) {
            clients.push(source);
        }

        let broadcast = format!("{}: {}", source, message);

        for client in &clients {
            socket.send_to(broadcast.as_bytes(), client).unwrap();
        }
    }
}
