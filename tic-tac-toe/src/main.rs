use engine::{Cell, Game, GameResult, DIM};
use std::io;
use std::io::Write;

fn parse_coords(s: &String, max_coord: usize) -> Result<(usize, usize), &'static str> {
    let mut parts = s.split_whitespace().map(|s| s.parse::<u32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b)))
            if a >= 1 && b >= 1 && a <= max_coord as u32 && b <= max_coord as u32 =>
        {
            Ok((
                usize::try_from(a - 1).unwrap(),
                usize::try_from(b - 1).unwrap(),
            ))
        }
        _ => Err("Невалидные координаты"),
    }
}

mod engine {
    use rand::Rng;

    pub const DIM: usize = 3;

    #[derive(Copy, Clone, PartialEq)]
    pub enum Cell {
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
        pub game: [[Cell; DIM]; DIM],
    }

    impl Game {
        pub fn new() -> Self {
            Game {
                game: [[Cell::Empty; DIM]; DIM],
            }
        }

        pub fn print(&self) {
            println!("------------");
            for y in 0..DIM {
                for x in 0..DIM {
                    match self.game[y][x] {
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
                if self.game[y].windows(2).all(|w| w[0] == w[1]) {
                    match self.game[y][0] {
                        Cell::Empty => continue,
                        Cell::Zero => return GameResult::ZeroWin,
                        Cell::Cross => return GameResult::CrossWin,
                    }
                }
            }

            for x in 0..DIM {
                let mut col = [Cell::Empty; DIM];
                for y in 0..DIM {
                    col[y] = self.game[y][x]
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
                diag[z] = self.game[z][z]
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
                diag[z] = self.game[z][2 - z]
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
                    if self.game[y][x] == Cell::Empty {
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
                    if self.game[y][x] == Cell::Empty {
                        empty_positions.push((x, y))
                    }
                }
            }
            let n_empty: u8 = empty_positions.len() as u8;
            if n_empty > 0 {
                let r_num: usize = rand::thread_rng().gen_range(0..n_empty) as usize;
                self.game[empty_positions[r_num].1][empty_positions[r_num].0] = Cell::Cross;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut game = Game::new();
    let (mut x, mut y);

    loop {
        print!("Ваш ход (x, y): ");
        io::stdout().flush().unwrap();

        input = "".to_string();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");

        let coords = parse_coords(&input, DIM);
        if !coords.is_ok() {
            println!(
                "Некорректный ввод. Введите два числа от 1 до {} через пробел.",
                DIM
            );
            continue;
        }
        (x, y) = coords.unwrap();

        if game.game[y][x] == Cell::Empty {
            game.game[y][x] = Cell::Zero
        } else {
            println!("Некорректный ввод. Клетка уже заполнена.")
        }
        game.make_action();
        game.print();

        let status = game.status();
        match status {
            GameResult::WaitNextStep => continue,
            GameResult::ZeroWin => {
                println!("Нолики победили! 🎉");
                break;
            }
            GameResult::CrossWin => {
                println!("Крестики победили! 🎉");
                break;
            }
            GameResult::Draw => {
                println!("Ничья 🤝");
                break;
            }
        }
    }
}
