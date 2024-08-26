pub mod board {
    use crate::tetromino::tetromino::{Color, Tetromino};
    // tetromino (file name) :: tetromino (module name)
    pub const BOARD_WIDTH: usize = 10;
    pub const BOARD_HEIGHT: usize = 40;
    pub const HIDDEN_ROWS: usize = 4;

    #[derive(Debug, Default, Copy, Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    #[derive(Debug)]
    pub struct Board {
        cells: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT],
    }

    impl Board {
        pub fn new() -> Self {
            Self {
                cells: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            }
        }

        pub fn lock_tetromino(&mut self, tetromino: &Tetromino, origin: Point) {
            tetromino.each_point(&mut |row, col| {
                let x = origin.x + (col as i32);
                let y = origin.y + (row as i32);
                self.cells[y as usize][x as usize] = Some(tetromino.color);
            });
        }

        pub fn collision_test(&self, tetromino: &Tetromino, origin: Point) -> bool {
            let mut collided = false;
            // funny lambda expression/closure/for loop
            tetromino.each_point(&mut |row, col| {
                // Once collided becomes true, the code below gets skipped over every iteration
                if !collided {
                    let x = origin.x + col;
                    let y = origin.y + row;
                    if x < 0
                        || x >= (BOARD_WIDTH as i32)
                        || y < 0
                        || y >= (BOARD_HEIGHT as i32)
                        || self.cells[y as usize][x as usize] != None
                    {
                        collided = true;
                    }
                }
            });
            collided
        }

        /// Clears the board of any complete lines, shifting down rows to take their place.
        /// Returns the total number of lines that were cleared.
        pub fn clear_lines(&mut self) -> usize {
            let mut cleared_lines: usize = 0;
            for row in (0..self.cells.len()).rev() {
                if (row as i32) - (cleared_lines as i32) < 0 {
                    break;
                }
                if cleared_lines > 0 {
                    self.cells[row] = self.cells[row - cleared_lines];
                    self.cells[row - cleared_lines] = [None; BOARD_WIDTH as usize];
                }
                while !self.cells[row].iter().any(|x| *x == None) {
                    cleared_lines += 1;
                    self.cells[row] = self.cells[row - cleared_lines];
                    self.cells[row - cleared_lines] = [None; BOARD_WIDTH as usize];
                }
            }
            cleared_lines as usize
        }

        pub fn add_lines(&mut self, lines: usize) -> usize {
            for row in (HIDDEN_ROWS..BOARD_HEIGHT).rev() {
                for col in 0..BOARD_WIDTH {
                    self.cells[row - lines][col] = self.cells[row][col];
                }
            }
            for row in 0..lines {
                for col in 0..BOARD_WIDTH {
                    self.cells[BOARD_HEIGHT - 1 - row][col] = Some(Color::Red);
                }
            }
            lines
        }

        pub fn lost(&self) -> bool {
            let mut lost = false;
            'outer: for row in 0..HIDDEN_ROWS {
                for col in 0..BOARD_WIDTH {
                    if self.cells[row][col] != None {
                        lost = true;
                        break 'outer;
                    }
                }
            }
            lost
        }

        // pub fn render(&self, display: &mut Display) {
        //     for y in HIDDEN_ROWS..BOARD_HEIGHT {
        //         display.set_text("|", 0, y, Color::Red, Color::Black);
        //         display.set_text("|", BOARD_WIDTH * 2 + 1, y, Color::Red, Color::Black);
        //     }
        //     for x in 0..(BOARD_WIDTH * 2 + 1) {
        //         display.set_text("-", x, BOARD_HEIGHT, Color::Red, Color::Black);
        //     }
        //     for row in 0..BOARD_HEIGHT {
        //         for col in 0..BOARD_WIDTH {
        //             match self.cells[row as usize][col as usize] {
        //                 Some(color) => {
        //                     let c = 1 + (col * 2);
        //                     display.set_text(" ", c, row, color, color);
        //                     display.set_text(" ", c + 1, row, color, color);
        //                 }
        //                 None => (),
        //             }
        //         }
        //     }
        // }
    }
}