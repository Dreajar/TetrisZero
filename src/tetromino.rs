pub mod tetromino {
    use rand::Rng;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Color {
        Black,
        Cyan,
        Purple,
        Green,
        Red,
        Blue,
        Orange,
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum Direction {
        CCW,
        CW,
    }
    #[derive(Debug)]
    pub struct Tetromino {
        pub color: Color,
        pub shape: Vec<Vec<u8>>,
    }
    impl Clone for Tetromino {
        fn clone(&self) -> Tetromino {
            let mut t = Tetromino {
                color: self.color,
                shape: Vec::with_capacity(self.shape.len()),
            };

            for row in &self.shape {
                t.shape.push(row.clone());
            }
            t
        }
    }

    impl Tetromino {
        pub fn new_o() -> Tetromino {
            Tetromino {
                color: Color::Cyan,
                shape: vec![vec![1, 1], vec![1, 1]],
            }
        }

        pub fn new_l() -> Tetromino {
            Tetromino {
                color: Color::Orange,
                shape: vec![vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
            }
        }

        pub fn new_j() -> Tetromino {
            Tetromino {
                color: Color::Blue,
                shape: vec![vec![1, 0, 0], vec![1, 1, 1], vec![0, 0, 0]],
            }
        }

        pub fn new_t() -> Tetromino {
            Tetromino {
                color: Color::Purple,
                shape: vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 0, 0]],
            }
        }

        pub fn new_s() -> Tetromino {
            Tetromino {
                color: Color::Green,
                shape: vec![vec![0, 1, 1], vec![1, 1, 0], vec![0, 0, 0]],
            }
        }

        pub fn new_z() -> Tetromino {
            Tetromino {
                color: Color::Red,
                shape: vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0]],
            }
        }

        pub fn new_i() -> Tetromino {
            Tetromino {
                color: Color::Black,
                shape: vec![
                    vec![0, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            }
        }

        pub fn rotate(&mut self, direction: Direction) {
            let size = self.shape.len();
            match direction {
                Direction::CCW => {
                    for row in 0..size / 2 {
                        for col in row..(size - row - 1) {
                            let t = self.shape[row][col];
                            self.shape[row][col] = self.shape[col][size - row - 1];
                            self.shape[col][size - row - 1] =
                                self.shape[size - row - 1][size - col - 1];
                            self.shape[size - row - 1][size - col - 1] =
                                self.shape[size - col - 1][row];
                            self.shape[size - col - 1][row] = t;
                        }
                    }
                }
                Direction::CW => {
                    for row in 0..size / 2 {
                        for col in row..(size - row - 1) {
                            let t = self.shape[row][col];
                            self.shape[row][col] = self.shape[size - col - 1][row];
                            self.shape[size - col - 1][row] =
                                self.shape[size - row - 1][size - col - 1];
                            self.shape[size - row - 1][size - col - 1] =
                                self.shape[col][size - row - 1];
                            self.shape[col][size - row - 1] = t;
                        }
                    }
                }
            }
        }

        // Rust needs the size of the type at compile time; otherwise it needs dyn or Box<>
        pub fn each_point(&self, f: &mut dyn FnMut(i32, i32)) {
            let n = self.shape.len() as i32;
            for row in 0..n {
                for col in 0..n {
                    if self.shape[row as usize][col as usize] != 0 {
                        f(row, col);
                    }
                }
            }
        }
    }

    // We use random permutations within groups of 7 to prevent certain variant droughts
    #[derive(Debug)]
    pub struct TetrominoQueue {
        pub tetrominoes: Vec<Tetromino>,
    }
    impl Default for TetrominoQueue {
        fn default() -> Self {
            TetrominoQueue::new()
        }
    }
    impl TetrominoQueue {
        pub fn new() -> TetrominoQueue {
            let mut q = TetrominoQueue {
                tetrominoes: Vec::new(),
            };
            q.add_seven();
            q
        }

        /// Returns a copy of the next tetromino in the queue.
        pub fn peek(&mut self) -> Tetromino {
            match self.tetrominoes.first() {
                Some(t) => t.clone(),
                None => {
                    self.add_seven();
                    self.peek()
                }
            }
        }

        /// Generates & adds random ordering of all variants to queue.
        pub fn add_seven(&mut self) {
            let mut all_variants: Vec<Tetromino> = vec![
                Tetromino::new_o(),
                Tetromino::new_l(),
                Tetromino::new_j(),
                Tetromino::new_t(),
                Tetromino::new_s(),
                Tetromino::new_z(),
                Tetromino::new_i(),
            ];

            while !self.tetrominoes.is_empty() {
                // gen_range(start..=end) is inclusive
                let i = rand::thread_rng().gen_range(0..=all_variants.len());
                self.tetrominoes.push(all_variants.swap_remove(i));
            }
        }
    }
}
