extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let filename = "/usr/share/dict/words";
    let mut random_line = get_random_line(filename).expect("Failed to read input data");
    while random_line.contains('\'') {
        random_line = get_random_line(filename).expect("Failed to read input data");
    }
    let mut gd : GameData = GameData {
        secret_line : random_line,
        discovered  : String::new(),
        lives       : 5,
        status      : String::new(),
    };

    let mut secret_line_masked = format_masked_string(&gd.secret_line, &gd.discovered);

    loop {
        update_screen(&gd, &secret_line_masked);

        println!("Type a guess:");
        let guess = read_guess();

        if validate_guess(guess) {
            let guess_lower = guess.unwrap().to_lowercase().next().unwrap();
            
            match check_guess(&gd, guess_lower) {
                UserInputStatus::LetterGuessed => {
                    gd.discovered.push(guess_lower);
                    gd.status = format!("You discovered {}", guess_lower);
                    secret_line_masked = format_masked_string(&gd.secret_line, &gd.discovered);

                    if !secret_line_masked.contains('_') {
                        gd.status = format!("You won!");
                        update_screen(&gd, &secret_line_masked);
                        break;
                    }
                },
                UserInputStatus::LetterMissed => {
                    gd.discovered.push(guess_lower);
                    gd.lives -= 1;
                    if gd.lives == 0 {
                        gd.status = format!("GAME OVER!");
                        secret_line_masked = format_masked_string(&gd.secret_line, &gd.secret_line);
                        update_screen(&gd, &secret_line_masked);
                        break;
                    }
                    else {
                        gd.status = format!("Nope, try again!");
                    }

                },
                UserInputStatus::AlreadyDiscovered => {
                    gd.status = format!("{} is already uncovered!", guess_lower);
                }
            }
        }
        else {
            gd.status = format!("Please input a valid alphabetic character");
        }
    }
}

fn update_screen(gd: &GameData, secret_line: &String) {
    println!("Hangman!");
    println!("Lives: {}. Discovered letters: {}", gd.lives, gd.discovered);
    println!("{}", secret_line);
    println!("{}", gd.status);
}

struct GameData {
    secret_line : String,
    discovered  : String,
    lives       : u32,
    status      : String,
}

enum UserInputStatus {
    AlreadyDiscovered,
    LetterGuessed,
    LetterMissed,
}

fn check_guess(gd : &GameData, guess : char) -> UserInputStatus {
    if gd.discovered.contains(guess) {
        return UserInputStatus::AlreadyDiscovered;
    }
    
    if gd.secret_line.contains(guess) {
        return UserInputStatus::LetterGuessed;
    }

    UserInputStatus::LetterMissed
}

fn format_masked_string(input: &String, mask: &String) -> String {
    let mut result = String::new();
    for (_, c) in input.chars().enumerate() {
        if c == ' ' || mask.contains(c) {
            result.push(c);
        }
        else {
            result.push('_');
        }
        result.push(' ');
    }
    result
}

fn read_guess() -> Option<char> {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().chars().nth(0)
}

fn validate_guess(guess: Option<char>) -> bool {
    match guess {
        None        => false,
        Some(guess) => guess.is_alphabetic(),
    }
}

fn get_random_line(filename : &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);
    let mut rng = rand::thread_rng();
    let sample = rand::sample(&mut rng, file.lines(), 1).pop().unwrap();
    let secret_line = sample.unwrap().to_lowercase();
    Ok(secret_line)
}
