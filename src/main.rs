mod header;
mod message;
mod question;

use message::DNSMessage;
use question::{DNSQuestion, QuestionClass, QuestionType};

use crate::header::DNSHeader;
use std::net::UdpSocket;

fn main() {
    let header = DNSHeader::new(1234, true, 9, false, false, false, false, 0, 0, 1, 0, 0, 0);

    let question = DNSQuestion {
        name: vec!["codecrafters".to_string(), "io".to_string()],
        qtype: QuestionType::A,
        class: QuestionClass::IN,
    };

    let message = DNSMessage {
        header,
        questions: vec![question],
    };

    println!("Logs from your program will appear here!");
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = message.encode();
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
