use colored::Colorize;
use rand::Rng;
use std::fs::read_to_string;
use std::io;
use std::io::Write;
use titlecase::titlecase;

const MAX_ATTEMPTS: usize = 5;
const WORDS_FILE: &str = "words.txt";
const DELIMITER: &str = "---";
const RIGHT_COLOR: &str = "cyan";
const ALMOST_RIGHT_COLOR: &str = "purple";

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn choose_target() -> String {
    let words = read_lines(WORDS_FILE);
    words[rand::thread_rng().gen_range(0..words.len())]
        .to_lowercase()
        .clone()
}

fn main() {
    let target: Vec<char> = choose_target().chars().collect();
    let mut attempts: usize = 1;

    while attempts <= MAX_ATTEMPTS {
        print!("Введите слово ({}/{}): ", attempts, MAX_ATTEMPTS);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");
        let guess: Vec<char> = input.trim().to_lowercase().chars().collect();

        if guess.len() != target.len() {
            println!("Ошибка. Введите слово из {} букв.", target.len());
            println!("{DELIMITER}");
            continue;
        }

        if guess == target {
            println!("{DELIMITER}");
            println!("Вы угадали! 🎉");
            break;
        }
        print!("Результат: ");

        for i in 0..guess.len() {
            if guess[i] == target[i] {
                print!("{}", guess[i].to_string().color(RIGHT_COLOR))
            } else if !target.iter().position(|&x| x == guess[i]).is_none() {
                print!("{}", guess[i].to_string().color(ALMOST_RIGHT_COLOR));
            } else {
                print!("{}", guess[i])
            }
        }

        println!("");
        println!("{DELIMITER}");

        attempts += 1;
    }
    println!(
        "Вы проиграли! Искомое слово: {}",
        titlecase(&String::from_iter(target.clone()))
    );
}
