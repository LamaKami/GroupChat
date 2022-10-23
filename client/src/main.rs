use core::time::Duration;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn get_valid_value_from_peer_addr(stream: &TcpStream) -> String {
    let peer_addr = stream.peer_addr();
    match peer_addr {
        Ok(v) => v.to_string(),
        Err(_) => String::from(""),
    }
}

fn group_chat_with_channels() {
    let (sender, receiver) = channel();

    let mut streams = Vec::new();
    thread::spawn(move || {
        handle_incoming_connections_with_channels(sender);
    });

    loop {
        let stream = receiver.recv_timeout(Duration::from_millis(10));
        if let Ok(v) = stream {
            streams.push(v)
        }
        streams.retain(|stream| stream.peer_addr().is_ok());
        let mut data = [0_u8; 256];
        let mut message_len = 0;
        let mut sender_id = String::from("");

        for mut stream in &streams {
            message_len = stream.read(&mut data).unwrap_or(0);
            if message_len != 0 {
                sender_id = get_valid_value_from_peer_addr(stream);
                break;
            }
        }

        if message_len != 0 {
            for mut stream in &streams {
                let peer_addr = get_valid_value_from_peer_addr(stream);

                if peer_addr != sender_id {
                    stream.write_all(&data).unwrap();
                }
            }
        }
    }
}

fn handle_incoming_connections_with_channels(sender: Sender<TcpStream>) {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::new(0, 1))).unwrap();
                stream.set_write_timeout(Some(Duration::new(0, 1))).unwrap();
                sender.send(stream).unwrap()
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn main() {
    group_chat_with_channels();
}
