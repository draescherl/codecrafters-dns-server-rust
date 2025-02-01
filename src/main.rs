mod answer;
mod dns_types;
mod header;
mod message;
mod question;

use message::DNSMessage;

use crate::answer::DNSAnswer;
use crate::dns_types::{DNSClass, DNSType};
use crate::header::DNSHeader;
use crate::question::DNSQuestion;
use std::net::UdpSocket;

fn main() {
    let header = DNSHeader::new(1234, true, 9, false, false, false, false, 0, 0, 1, 1, 0, 0);

    let question = DNSQuestion {
        name: vec!["codecrafters".to_string(), "io".to_string()],
        question_type: DNSType::A,
        class: DNSClass::IN,
    };

    let answer = DNSAnswer {
        name: vec!["codecrafters".to_string(), "io".to_string()],
        answer_type: DNSType::A,
        class: DNSClass::IN,
        ttl: 60,
        rd_length: 4,
        r_data: vec![8, 8, 8, 8],
    };

    let message = DNSMessage {
        header,
        questions: vec![question],
        answers: vec![answer],
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
