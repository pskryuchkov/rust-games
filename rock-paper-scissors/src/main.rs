use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::Rng;
use std::io;
use std::io::Write;

#[derive(FromPrimitive, PartialEq)]
enum Action {
    Rock = 0,
    Paper,
    Scissors,
}

fn char_to_action(c: char) -> Action {
    match c {
        'к' => Action::Rock,
        'н' => Action::Paper,
        'б' => Action::Scissors,
        _ => panic!("Неверный диапазон"),
    }
}

fn print_opponent_action(a: &Action) {
    match a {
        Action::Rock => println!("Мой предмет: Камень 🪨"),
        Action::Paper => println!("Мой предмет: Бумага 📄"),
        Action::Scissors => println!("Мой предмет: Ножницы ✂️"),
    }
}

fn is_win(user_action: &Action, _machine_action: &Action) -> bool {
    *user_action == Action::Paper && *_machine_action == Action::Rock
        || *user_action == Action::Scissors && *_machine_action == Action::Paper
        || *user_action == Action::Rock && *_machine_action == Action::Scissors
}

fn main() {
    loop {
        print!("Ваш предмет (К)амень/(Н)ожницы/(Б)умага: ");
        io::stdout().flush().unwrap();

        let mut action = String::new();

        io::stdin().read_line(&mut action).expect("Ошибка ввода");

        let input: char = action.trim().parse().expect("Неверный ввод");

        let user_action: Action = char_to_action(input);

        let _random_num: i32 = rand::thread_rng().gen_range(0..3);
        let _machine_action: Action = FromPrimitive::from_i32(_random_num).unwrap();

        print_opponent_action(&_machine_action);

        if is_win(&user_action, &_machine_action) {
            println!("Итог: Вы победили! 🎉")
        } else if matches!(user_action, _machine_action) {
            println!("Итог: Ничья 🤷")
        } else {
            println!("Итог: Вы проиграли 😢")
        }
        println!("---")
    }
}
