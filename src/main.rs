mod answer;
mod dns_types;
mod header;
mod message;
mod question;

use message::DNSMessage;

use std::net::UdpSocket;

fn main() {
    println!("Logs from your program will appear here!");
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let query_message = DNSMessage::parse(&buf);
                let answer_message = query_message.reply();
                let response = answer_message.encode();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
