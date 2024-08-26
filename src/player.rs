pub mod player {
    use anyhow::Result;
    use std::sync::Arc;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
        sync::RwLock,
    };
    // use tokio_tungstenite;

    pub enum Key {
        Up,
        Down,
        Left,
        Right,
        Space,
        CtrlC,
        Char(char),
    }

    #[derive(Debug)]
    pub struct Player {
        name: String,
        stream: Arc<RwLock<TcpStream>>,
    }
    // Shut down the TCP connection when the object goes out of scope.
    impl Drop for Player {
        fn drop(&mut self) {
            println!("[+] Ending TCP connection");
        }
    }
    impl Player {
        pub fn new(name: String, stream: Arc<RwLock<TcpStream>>) -> Self {
            Self { name, stream }
        }

        pub async fn send(&mut self, msg: String) -> Result<()> {
            let mut cstream = self.stream.write().await;
            cstream.write_all(msg.as_bytes()).await?;
            cstream.flush().await?;
            Ok(())
        }

        pub async fn keypress(&mut self, key: Key) {
            let _ = match key {
                Key::Left => self.send("Left".to_string()).await,
                Key::Right => self.send("Right".to_string()).await,
                Key::Char('q') => self.send("CCW".to_string()).await,
                Key::Char('e') => self.send("CW".to_string()).await,
                Key::Space => self.send("Space".to_string()).await,
                _ => self.send("Nothing".to_string()).await, // TODO: Look up how to fix this
            };
        }
        // Reads the) next line from the TCP stream.
        //     pub async fn read(&mut self) -> Result<String> {
        //         let cstream = self.stream.clone(); // Acquire read lock
        //         let mut stream_lock = cstream.write().await;
        //         let mut buffer = Vec::new();
        //         stream_lock.read_to_end(&mut buffer).await?;
        //         Ok(String::from_utf8(buffer).unwrap())
        //     }
        // }
    }
    pub fn get_input(stdin: &mut std::io::Stdin) -> Option<Key> {
        use std::io::Read;

        let c = &mut [0u8];
        match stdin.read(c) {
            Ok(_) => {
                match std::str::from_utf8(c) {
                    Ok("w") => Some(Key::Up),
                    Ok("a") => Some(Key::Left),
                    Ok("s") => Some(Key::Down),
                    Ok("d") => Some(Key::Right),
                    Ok(" ") => Some(Key::Space),
                    Ok("\x03") => Some(Key::CtrlC),
                    // Escape sequence started - must read two more bytes.
                    Ok("\x1b") => {
                        let code = &mut [0u8; 2];
                        match stdin.read(code) {
                            Ok(_) => match std::str::from_utf8(code) {
                                Ok("[A") => Some(Key::Up),
                                Ok("[B") => Some(Key::Down),
                                Ok("[C") => Some(Key::Right),
                                Ok("[D") => Some(Key::Left),
                                _ => None,
                            },
                            Err(msg) => panic!("could not read from standard in: {}", msg),
                        }
                    }
                    Ok(n) => Some(Key::Char(n.chars().next().unwrap())),
                    _ => None,
                }
            }
            Err(msg) => panic!("could not read from standard in: {}", msg),
        }
    }
}
