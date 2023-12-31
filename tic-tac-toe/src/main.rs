use rand::Rng;
use std::io;
use std::io::Write;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty = 0,
    Zero,
    Cross,
}

enum GameResult {
    WaitNextStep,
    ZeroWin,
    CrossWin,
    Draw,
}

fn parse_coords(s: &String) -> Result<(usize, usize), &'static str> {
    let mut parts = s.split_whitespace().map(|s| s.parse::<u32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) if a >= 1 && b >= 1 && a <= 3 && b <= 3 => Ok((
            usize::try_from(a - 1).unwrap(),
            usize::try_from(b - 1).unwrap(),
        )),
        _ => Err("Невалидные координаты"),
    }
}

fn print_game(game: [[Cell; 3]; 3]) {
    println!("------------");
    for y in 0..3 {
        for x in 0..3 {
            match game[y][x] {
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

fn check_game(game: [[Cell; 3]; 3]) -> GameResult {
    for y in 0..3 {
        if game[y].windows(2).all(|w| w[0] == w[1]) {
            match game[y][0] {
                Cell::Empty => continue,
                Cell::Zero => return GameResult::ZeroWin,
                Cell::Cross => return GameResult::CrossWin,
            }
        }
    }

    for x in 0..3 {
        let mut col = [Cell::Empty; 3];
        for y in 0..3 {
            col[y] = game[y][x]
        }

        if col.windows(2).all(|w| w[0] == w[1]) {
            match col[0] {
                Cell::Empty => continue,
                Cell::Zero => return GameResult::ZeroWin,
                Cell::Cross => return GameResult::CrossWin,
            }
        }
    }

    let mut diag = [Cell::Empty; 3];
    for z in 0..3 {
        diag[z] = game[z][z]
    }
    if diag.windows(2).all(|w| w[0] == w[1]) {
        match diag[0] {
            Cell::Empty => {}
            Cell::Zero => return GameResult::ZeroWin,
            Cell::Cross => return GameResult::CrossWin,
        }
    }

    let mut diag = [Cell::Empty; 3];
    for z in 0..3 {
        diag[z] = game[z][2 - z]
    }
    if diag.windows(2).all(|w| w[0] == w[1]) {
        match diag[0] {
            Cell::Empty => {}
            Cell::Zero => return GameResult::ZeroWin,
            Cell::Cross => return GameResult::CrossWin,
        }
    }

    for y in 0..3 {
        for x in 0..3 {
            if game[y][x] == Cell::Empty {
                return GameResult::WaitNextStep;
            }
        }
    }

    GameResult::Draw
}

fn make_action(mut game: [[Cell; 3]; 3]) -> [[Cell; 3]; 3] {
    let mut empty_positions: Vec<(usize, usize)> = Vec::new();
    for y in 0..3 {
        for x in 0..3 {
            if game[y][x] == Cell::Empty {
                empty_positions.push((x, y))
            }
        }
    }
    let n_empty: u8 = empty_positions.len() as u8;
    if n_empty > 0 {
        let r_num: usize = rand::thread_rng().gen_range(0..n_empty) as usize;
        game[empty_positions[r_num].1][empty_positions[r_num].0] = Cell::Cross;
    }
    game
}

fn main() {
    let mut input = String::new();
    let mut game = [[Cell::Empty; 3]; 3];
    let (mut x, mut y);

    loop {
        print!("Ваш ход (x, y): ");
        io::stdout().flush().unwrap();

        input = "".to_string();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");

        let coords = parse_coords(&input);
        if !coords.is_ok() {
            println!("Некорректный ввод. Введите два числа от 1 до 3 через пробел.");
            continue;
        }
        (x, y) = coords.unwrap();

        if game[y][x] == Cell::Empty {
            game[y][x] = Cell::Zero
        } else {
            println!("Некорректный ввод. Клетка уже заполнена.")
        }
        game = make_action(game);
        print_game(game);

        let status = check_game(game);
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
