pub mod engine {
    use rand::Rng;

    pub const DIM: usize = 3;

    #[derive(Copy, Clone, PartialEq)]
    enum Cell {
        Empty = 0,
        Zero,
        Cross,
    }

    pub enum GameResult {
        WaitNextStep,
        ZeroWin,
        CrossWin,
        Draw,
    }

    pub struct Game {
        state: [[Cell; DIM]; DIM],
    }

    impl Game {
        pub fn new() -> Self {
            Game {
                state: [[Cell::Empty; DIM]; DIM],
            }
        }

        pub fn print(&self) {
            println!("------------");
            for y in 0..DIM {
                for x in 0..DIM {
                    match self.state[y][x] {
                        Cell::Empty => print!("   |"),
                        Cell::Cross => print!(" x |"),
                        Cell::Zero => print!(" o |"),
                        _ => continue,
                    }
                }
                println!("")
            }
            println!("------------");
        }

        pub fn status(&self) -> GameResult {
            for y in 0..DIM {
                if self.state[y].windows(2).all(|w| w[0] == w[1]) {
                    match self.state[y][0] {
                        Cell::Empty => continue,
                        Cell::Zero => return GameResult::ZeroWin,
                        Cell::Cross => return GameResult::CrossWin,
                    }
                }
            }

            for x in 0..DIM {
                let mut col = [Cell::Empty; DIM];
                for y in 0..DIM {
                    col[y] = self.state[y][x]
                }

                if col.windows(2).all(|w| w[0] == w[1]) {
                    match col[0] {
                        Cell::Empty => continue,
                        Cell::Zero => return GameResult::ZeroWin,
                        Cell::Cross => return GameResult::CrossWin,
                    }
                }
            }

            let mut diag = [Cell::Empty; DIM];
            for z in 0..DIM {
                diag[z] = self.state[z][z]
            }
            if diag.windows(2).all(|w| w[0] == w[1]) {
                match diag[0] {
                    Cell::Empty => {}
                    Cell::Zero => return GameResult::ZeroWin,
                    Cell::Cross => return GameResult::CrossWin,
                }
            }

            let mut diag = [Cell::Empty; DIM];
            for z in 0..DIM {
                diag[z] = self.state[z][DIM - 1 - z]
            }
            if diag.windows(2).all(|w| w[0] == w[1]) {
                match diag[0] {
                    Cell::Empty => {}
                    Cell::Zero => return GameResult::ZeroWin,
                    Cell::Cross => return GameResult::CrossWin,
                }
            }

            for y in 0..DIM {
                for x in 0..DIM {
                    if self.state[y][x] == Cell::Empty {
                        return GameResult::WaitNextStep;
                    }
                }
            }

            GameResult::Draw
        }

        pub fn make_action(&mut self) {
            let mut empty_positions: Vec<(usize, usize)> = Vec::new();
            for y in 0..DIM {
                for x in 0..DIM {
                    if self.state[y][x] == Cell::Empty {
                        empty_positions.push((x, y))
                    }
                }
            }
            let n_empty: u8 = empty_positions.len() as u8;
            if n_empty > 0 {
                let r_num: usize = rand::thread_rng().gen_range(0..n_empty) as usize;
                self.state[empty_positions[r_num].1][empty_positions[r_num].0] = Cell::Cross;
            }
        }

        pub fn is_empty(&self, x: usize, y: usize) -> bool {
            self.state[y][x] == Cell::Empty
        }

        pub fn set(&mut self, x: usize, y: usize) {
            self.state[y][x] = Cell::Zero
        }
    }
}
