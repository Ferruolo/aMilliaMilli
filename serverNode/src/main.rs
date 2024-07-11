use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, thread};
use std::time::Duration;

mod parser;

const N_THREADS: usize = 32;
enum Command {
    GET,
    SET,
    KILL,
    Unknown,
}

fn handle_client(mut stream: TcpStream, running: Arc<AtomicBool>) {
    let mut buffer = [0; 1024];

    while running.load(Ordering::Relaxed) {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    return;
                }

                let received = String::from_utf8_lossy(&buffer[..size]);
                let command = match received.trim().to_uppercase().as_str() {
                    "GET" => Command::GET,
                    "SET" => Command::SET,
                    "KILL" => Command::KILL,
                    _ => Command::Unknown,
                };

                match command {
                    Command::GET => {
                        println!("Received GET command");
                        let _ = stream.write_all(b"Received GET command\n");
                    }
                    Command::SET => {
                        println!("Received SET command");
                        let _ = stream.write_all(b"Received SET command\n");
                    }
                    Command::KILL => {
                        println!("Received KILL command, initiating shutdown");
                        let _ = stream.write_all(b"Server shutting down\n");
                        running.store(false, Ordering::Relaxed);
                    }
                    Command::Unknown => {
                        println!("Received unknown command");
                        let _ = stream.write_all(b"Unknown command\n");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from connection: {}", e);
                return;
            }
        }
    }
    stream.shutdown(Shutdown::Both).unwrap();
    println!("Process Killed");
}

// Test TCP connection wit h
// nc 127.0.0.1 8080
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    listener.set_nonblocking(true)?;
    println!("Server listening on 0.0.0.0:8080");

    let running = Arc::new(AtomicBool::new(true));

    let mut handles = vec![];


    while running.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let running_clone = Arc::clone(&running);
                handles.push(thread::spawn(move || {
                    handle_client(stream, running_clone);
                    println!("Ending thread")
                }));
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // Wait a bit before trying again
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }

    println!("Shutting down. Waiting for all connections to close...");
    for handle in handles {
        println!("Killing a handle");
        let _ = handle.join().unwrap();
        println!("He Dead. Amen");
    }

    Ok(())
}

