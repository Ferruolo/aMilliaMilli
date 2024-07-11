use std::io::ErrorKind;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, thread};
use std::time::Duration;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    listener.set_nonblocking(true)?;
    println!("Server listening on 0.0.0.0:8080");

    let running = Arc::new(AtomicBool::new(true));

    while running.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                match stream.peer_addr() {
                    Ok(s) => {
                        let ip_addr = s.ip().to_string();
                        println!("Received stream from address {}", ip_addr);
                    }
                    Err(e) => {
                        println!("Received stream but got error: {e}")
                    }
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // Wait a bit before trying again
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }
    Ok(())
}

