use core::time::Duration;
use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::sync::mpsc::{channel, Sender};
use std::thread;
fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let (sender, receiver) = channel();
            let client_name = stream.local_addr().unwrap().to_string();
            thread::spawn(move || {
                read_user_input(sender, client_name);
            });
            stream
                .set_read_timeout(Some(Duration::new(0, 100)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::new(0, 100)))
                .unwrap();
            loop {
                let mut data = [0_u8; 256];
                if stream.read(&mut data).is_ok() {
                    let new_msg = from_utf8(&data).unwrap();
                    println!("{}", new_msg);
                }

                let received_msg = receiver
                    .recv_timeout(Duration::from_millis(100))
                    .unwrap_or_else(|_| String::from(""));
                if received_msg.is_empty() {
                    continue;
                }
                let msg = received_msg.as_bytes();
                let _ = stream.write(msg).unwrap();
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn read_user_input(sender: Sender<String>, addr: String) {
    loop {
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        sender
            .send(String::from("Client ") + &addr + ": " + &s)
            .unwrap();
    }
}
