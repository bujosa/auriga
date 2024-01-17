use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, store: Arc<Mutex<HashMap<String, String>>>) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let msg = String::from_utf8_lossy(&buffer[..]);
    let mut parts = msg.split_whitespace();
    let command = parts.next();
    let key = parts.next();
    let value = parts.next();

    match (command, key, value) {
        (Some("GET"), Some(key), None) => {
            if let Some(value) = store.lock().unwrap().get(key) {
                stream.write(value.as_bytes())?;
            }
        }
        (Some("SET"), Some(key), Some(value)) => {
            store.lock().unwrap().insert(key.to_string(), value.to_string());
            stream.write(b"OK")?;
        }
        _ => {
            stream.write(b"ERROR")?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    let store = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || {
                    handle_client(stream, store).unwrap();
                });
            }
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
            }
        }
    }

    Ok(())
}