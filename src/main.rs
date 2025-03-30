mod answer;
mod dns_types;
mod header;
mod message;
mod name;
mod question;

use message::DNSMessage;

use crate::answer::DNSAnswer;
use crate::header::DNSHeader;
use std::{env, net::UdpSocket};

fn main() {
    let args: Vec<String> = env::args().collect();
    let resolver: Option<String> = if args.len() > 1 && args[1] == "--resolver" {
        Some(args[2].clone())
    } else {
        None
    };

    let listen_addr = "127.0.0.1:2053";
    println!("Listening on {}", listen_addr);
    let udp_socket = UdpSocket::bind(listen_addr).expect("Failed to bind to address");

    let bind_addr = "127.0.0.1:2054";
    let resolver_socket = UdpSocket::bind(bind_addr).expect("Failed to bind address");

    let mut buf = [0; 512];
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let query_message = DNSMessage::parse(&buf);
                let answer_message = if let Some(ref resolver_addr) = resolver {
                    let queries = query_message.split_questions();
                    let mut answers: Vec<DNSAnswer> = vec![];
                    let mut reference_header: DNSHeader = query_message.header.clone();
                    for query in &queries {
                        resolver_socket
                            .send_to(&query.encode(), &resolver_addr)
                            .expect("Failed to send response");
                        let mut response = match resolver_socket.recv_from(&mut buf) {
                            Ok(_) => DNSMessage::parse(&buf),
                            Err(e) => {
                                eprintln!("Error receiving data: {}", e);
                                break;
                            }
                        };
                        answers.append(&mut response.answers);
                        reference_header = response.header;
                    }
                    DNSMessage::merge(reference_header, query_message.questions, answers)
                } else {
                    query_message.reply()
                };
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
