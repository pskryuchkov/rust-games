mod core;

use core::{Game, GAME_DIM};
use std::io;
use std::io::Write;

fn parse_coords(s: &String, max_coord: usize) -> Result<(usize, usize), &'static str> {
    let mut parts = s.split_whitespace().map(|s| s.parse::<u32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b)))
            if a >= 1 && b >= 1 && a <= max_coord as u32 && b <= max_coord as u32 =>
        {
            Ok((
                usize::try_from(a).unwrap() - 1,
                usize::try_from(b).unwrap() - 1,
            ))
        }
        _ => Err("Невалидные координаты"),
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        print!("Ваш ход (x, y): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");

        let coords = parse_coords(&input, GAME_DIM);
        if !coords.is_ok() {
            println!(
                "Некорректный ввод. Введите два числа от 1 до {} через пробел.",
                GAME_DIM
            );
            continue;
        }
        let (x, y) = coords.unwrap();

        if game.is_empty_cell(x, y) {
            game.set(x, y)
        } else {
            println!("Некорректный ввод. Клетка уже заполнена.")
        }
        game.make_action();
        game.print_state();

        let status = game.status();
        match game.is_continued(&status) {
            true => continue,
            false => {
                game.print_result(status);
                break;
            }
        }
    }
}
