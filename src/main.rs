use std::collections::HashMap;
use std::io::{BufRead, Write, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(stream: TcpStream, store: Arc<Mutex<HashMap<String, String>>>) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return;
                }

                let command: Vec<&str> = buffer.split_whitespace().collect();
                match command.as_slice() {
                    ["SET", key, value] => {
                        let mut store = store.lock().unwrap();
                        store.insert(key.to_string(), value.to_string());
                        if let Err(e) = writeln!(writer, "OK") {
                            eprintln!("Error writing to client: {}", e);
                        }
                    }
                    ["GET", key] => {
                        let store = store.lock().unwrap();
                        match store.get(*key) {
                            Some(value) => {
                                if let Err(e) = writeln!(writer, "{}", value) {
                                    eprintln!("Error writing to client: {}", e);
                                }
                            }
                            None => {
                                if let Err(e) = writeln!(writer, "ERROR: Key not found") {
                                    eprintln!("Error writing to client: {}", e);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Err(e) = writeln!(writer, "ERROR: Unknown command") {
                            eprintln!("Error writing to client: {}", e);
                        }
                    }
                }
                if let Err(e) = writer.flush() {
                    eprintln!("Error flushing writer: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                return;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6390")?;
    let store = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {}", stream.peer_addr().unwrap());
                let store = Arc::clone(&store);
                thread::spawn(move || {
                    handle_client(stream, store);
                });
            }
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
            }
        }
    }

    Ok(())
}