pub mod game {
    use crate::board::board::{Board, Point, BOARD_WIDTH};
    use crate::player::player::Player;
    use crate::tetromino::tetromino::{Direction, TetrominoQueue};
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Game {
        pub players: HashMap<usize, Player>, // Maybe delete this line
        pub boards: HashMap<usize, Board>,
        pub tetromino_queue: TetrominoQueue,
        pub current_tetrominoes: HashMap<usize, usize>,
        pub tetromino_positions: HashMap<usize, Point>,
        pub survivors: HashMap<usize, bool>,
    }
    // Annonymous lifetime
    impl Game {
        pub fn start(&mut self) {
            // Display graphics
            for (id, _) in self.survivors.clone() {
                self.advance_game(&id, &mut 0usize);
            }
        }

        // Each movement function generates a fake tetromino in the specified position.
        // Then, the function (conditionally) updates the position & returns !collided.
        pub fn move_tetromino(&mut self, id: &usize, x: i32, y: i32) -> bool {
            let new_position = Point {
                x: self.tetromino_positions[id].x + x,
                y: self.tetromino_positions[id].y + y,
            };
            let collided = self.boards[id].collision_test(
                &self.tetromino_queue.tetrominoes[self.current_tetrominoes[id]],
                new_position,
            );
            // pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
            if !collided {
                *self.tetromino_positions.get_mut(id).unwrap() = new_position;
            }
            !collided
        }

        // TODO: Currently players can rotate each others' pieces LMFAO
        pub fn rotate_tetromino(&mut self, id: &usize, direction: Direction) -> bool {
            let mut new_tetromino =
                self.tetromino_queue.tetrominoes[self.current_tetrominoes[id]].clone();
            new_tetromino.rotate(direction);
            let collided =
                self.boards[id].collision_test(&new_tetromino, self.tetromino_positions[id]);
            if !collided {
                self.tetromino_queue.tetrominoes[*id] = new_tetromino;
            }
            !collided
        }

        // Places current tetromino at the top of the board
        pub fn place_new_tetromino(&mut self, id: &usize) -> bool {
            let origin = Point {
                x: ((BOARD_WIDTH
                    - (self.tetromino_queue.tetrominoes[self.current_tetrominoes[id]]
                        .shape.len() as usize)) / 2) as i32,
                y: 0,
            };
            let collided = self.boards[id].collision_test(
                &self.tetromino_queue.tetrominoes[self.current_tetrominoes[id]],
                origin,
            );
            if !collided {
                *self.tetromino_positions.get_mut(id).unwrap() = origin;
            }
            !collided
        }

        // Drops current tetromino to lowest spot on board (fits w/o collisions); advances game
        pub fn drop_tetromino(&mut self, id: &usize) -> bool {
            while self.move_tetromino(id, 0, 1) {}
            self.advance_game(id, &mut 0usize)
        }

        // Either moves tetromino down 1 or places new tetromino at the top & checks if alive
        pub fn advance_game(&mut self, id: &usize, cleared_lines: &mut usize) -> bool {
            let moved = self.move_tetromino(id, 0, 1);
            if moved {
                return true;
            } else {
                self.boards.get_mut(id).unwrap().lock_tetromino(
                    &self.tetromino_queue.tetrominoes[self.current_tetrominoes[id]],
                    self.tetromino_positions[id],
                );
                *cleared_lines = self.boards.get_mut(id).unwrap().clear_lines();
                *self.current_tetrominoes.get_mut(id).unwrap() += 1;
                return self.place_new_tetromino(id); // Can't place -> lost
            }
        }

        ///////////////////////////////////////////
        // fn play(&mut self, display: &mut Display) {
        //     let (tx_event, rx_event) = mpsc::channel();

        //     // Spawn a thread which sends periodic game ticks to advance the tetromino
        //     {
        //         let tx_event = tx_event.clone();
        //         thread::spawn(move || loop {
        //             thread::sleep(Duration::from_millis(500));
        //             tx_event.send(GameUpdate::Tick).unwrap();
        //         });
        //     }

        //     // Spawn a thread which listens for keyboard input
        //     {
        //         let tx_event = tx_event.clone();
        //         thread::spawn(move || {
        //             let stdin = &mut std::io::stdin();

        //             loop {
        //                 match get_input(stdin) {
        //                     Some(k) => tx_event.send(GameUpdate::KeyPress(k)).unwrap(),
        //                     None => (),
        //                 }
        //             }
        //         });
        //     }

        //     // Main game loop. The loop listens and responds to timer and keyboard updates received on a channel
        //     // as sent by the threads spawned above.
        //     loop {
        //         display.clear_buffer();
        //         self.render(display);
        //         display.render();

        //         match rx_event.recv() {
        //             Ok(update) => {
        //                 match update {
        //                     GameUpdate::KeyPress(key) => {
        //                         match key {
        //                             Key::Char('z') | Key::CtrlC => break,
        //                             k => {
        //                                 self.keypress(k);
        //                             }
        //                         };
        //                     }
        //                     GameUpdate::Tick => {
        //                         self.advance_game();
        //                     }
        //                 };
        //             }
        //             Err(err) => panic!(err),
        //         }
        //     }
        // }

        /// Shows the grid of the players.
        ///
        /// Hits/misses are shown on the upper grid.
        /// Lower grid is used for showing the player ships.

        /// Draws the game to the display.
        // fn render(&self, display: &mut Display) {
        //     // Render the board
        //     self.board.render(display);

        //     // Render the level
        //     let left_margin = BOARD_WIDTH * 2 + 5;
        //     display.set_text("Level: 1", left_margin, 3, Color::Red, Color::Black);

        //     // Render the currently falling tetromino
        //     let x = 1 + (2 * self.tetromino_position.x);
        //     self.render_tetromino(
        //         display,
        //         &self.tetromino,
        //         Point {
        //             x: x,
        //             y: self.tetromino_position.y,
        //         },
        //     );

        //     // Render the next tetromino
        //     display.set_text("Next tetromino:", left_margin, 7, Color::Red, Color::Black);
        //     let next_tetromino = self.tetromino_queue.peek();
        //     self.render_tetromino(
        //         display,
        //         &next_tetromino,
        //         Point {
        //             x: (left_margin as i32) + 2,
        //             y: 9,
        //         },
        //     );
        // }

        // fn render_tetromino(&self, display: &mut Display, tetromino: &Tetromino, origin: Point) {
        //     let color = tetromino.color;

        //     tetromino.each_point(&mut |row, col| {
        //         let x = (origin.x + 2 * col) as u32;
        //         let y = (origin.y + row) as u32;
        //         display.set_text(" ", x, y, color, color);
        //         display.set_text(" ", x + 1, y, color, color);
        //     });
        // }

        pub fn add_player(&mut self, id: usize, player: Player) {
            self.players.insert(id, player);
            self.boards.insert(id, Board::new());
            self.current_tetrominoes.insert(id, 0);
            self.tetromino_positions.insert(id, Point { x: 0, y: 0 });
            self.survivors.insert(id, true);
        }
    }
}
