use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub struct SingleInstance;

impl SingleInstance {
    pub fn is_single() -> bool {
        let socket_path = match Self::get_socket_path("calm") {
            Ok(path) => path,
            Err(_) => return true,
        };

        if !socket_path.exists() {
            return true;
        }

        match UnixStream::connect(&socket_path) {
            Ok(_) => false,
            Err(_) => {
                let _ = std::fs::remove_file(&socket_path);
                true
            }
        }
    }

    pub fn send_to_existing(url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let socket_path = Self::get_socket_path("calm")?;
        let mut stream = UnixStream::connect(&socket_path)?;
        stream.write_all(url.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
        Ok(())
    }

    pub fn start_listener() -> Result<Receiver<String>, Box<dyn std::error::Error>> {
        let socket_path = Self::get_socket_path("calm")?;

        let listener = match UnixListener::bind(&socket_path) {
            Ok(l) => l,
            Err(_) if socket_path.exists() => {
                std::fs::remove_file(&socket_path)?;
                UnixListener::bind(&socket_path)?
            }
            Err(e) => return Err(e.into()),
        };
        let (tx, rx) = channel();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut buffer = String::new();
                        if stream.read_to_string(&mut buffer).is_ok() {
                            for line in buffer.lines() {
                                if !line.is_empty() && tx.send(line.to_string()).is_err() {
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });

        Ok(rx)
    }

    fn get_socket_path(app_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .or_else(|_| std::env::var("TMPDIR"))
            .unwrap_or_else(|_| "/tmp".to_string());

        Ok(PathBuf::from(runtime_dir).join(format!("{}.socket", app_name)))
    }
}
