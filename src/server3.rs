pub mod server3 {
    use crate::game::game::Game;
    use crate::player::player::Player;
    use crate::tetromino::tetromino::Direction;
    use anyhow::Result;
    use std::{collections::HashMap, sync::Arc};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
        sync::{Notify, RwLock},
        time::{sleep, Duration},
    };

    pub async fn start_game_server(socket_addr: &str, max_players: usize) -> Result<()> {
        let listener = TcpListener::bind(socket_addr).await.unwrap();
        println!("[+] Server is listening on {}", socket_addr);
        let connections = Arc::new(RwLock::new(HashMap::new()));
        let game = Arc::new(RwLock::new(Game::default()));
        let notifier = Arc::new(Notify::new());
        let player_count = Arc::new(RwLock::new(0));

        // listener.accept() is the one that keeps returning shit;
        // while just continuously pattern matches
        'accept_while: while let Ok((stream, _)) = listener.accept().await {
            *player_count.write().await += 1;

            if *player_count.read().await > max_players {
                send(
                    Arc::new(RwLock::new(stream)),
                    String::from("Lobby is currently full"),
                )
                .await?;
                break 'accept_while;
            }

            println!("[+] New connection: {}", stream.peer_addr()?);
            let shared_stream = Arc::new(RwLock::new(stream));
            let cloned_stream = shared_stream.clone();
            let new_game = game.clone();
            let new_notifier = notifier.clone();
            let id = *player_count.read().await + 1; // 1-based
            (*(connections.write().await)).insert(id, cloned_stream);
            send(
                shared_stream.clone(),
                String::from("Enter your name below:"),
            )
            .await?;
            let name = read(shared_stream.clone()).await.unwrap();
            (*(game.write().await)).add_player(id, Player::new(name, shared_stream.clone()));
            if *player_count.read().await == max_players {
                (*(game.write().await)).start();
                notifier.notify_one();
            }

            tokio::spawn(async move {
                new_notifier.notified().await;
                'alive: loop {
                    let msg = read(shared_stream.clone())
                        .await
                        .unwrap()
                        .trim()
                        .to_string();
                    process_message(&id, &msg, new_game.clone()).await;
                    if !(*(new_game.read().await)).survivors[&id] {
                        break 'alive;
                    }
                    sleep(Duration::from_millis(100)).await;
                }
            });
        }
        Ok(())
    }

    async fn process_message(id: &usize, msg: &str, game: Arc<RwLock<Game>>) -> bool {
        // TODO: Check if dead
        match msg {
            "Left" => (*(game.write().await)).move_tetromino(id, -1, 0),
            "Right" => (*(game.write().await)).move_tetromino(id, 1, 0),
            "CCW" => (*(game.write().await)).rotate_tetromino(id, Direction::CCW),
            "RotateRight" => (*(game.write().await)).rotate_tetromino(id, Direction::CW),
            "Space" => {
                (*(game.write().await)).drop_tetromino(id);
                *((*(game.write().await))
                    .current_tetrominoes
                    .get_mut(id)
                    .unwrap()) += 1;

                // locks the tetromino and clears lines
                let mut cleared_lines = 0usize;
                let alive = (*(game.write().await)).advance_game(id, &mut cleared_lines);
                if !alive {
                    *((*(game.write().await)).survivors.get_mut(id).unwrap()) = false;
                } else if cleared_lines > 0 {
                    for (pid, _) in (*(game.read().await)).survivors.clone() {
                        (*(game.write().await))
                            .boards
                            .get_mut(&pid)
                            .unwrap()
                            .add_lines(pid);
                    }
                }
                true
            }
            _ => false,
        }
    }

    pub async fn send(arc_stream: Arc<RwLock<TcpStream>>, msg: String) -> Result<()> {
        let mut stream = arc_stream.write().await;
        stream.write_all(msg.as_bytes()).await?;
        stream.flush().await?;
        Ok(())
    }

    pub async fn read(arc_stream: Arc<RwLock<TcpStream>>) -> Result<String> {
        let stream = arc_stream.clone(); // Acquire read lock
        let mut stream_lock = stream.write().await;
        let mut buffer = Vec::new();
        stream_lock.read_to_end(&mut buffer).await?;
        Ok(String::from_utf8(buffer).unwrap())
    }
}
