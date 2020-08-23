use std::io;

use rand;
use rand::Rng;

const MIN: u8 = 0;
const MAX: u8 = 100;
const MAX_ATTEMPTS: u8 = 7;

fn main() {
    let secret_number = get_secret_number();
    start_game(secret_number);
}

fn start_game(secret_number: u8) {
    // println!("(secret number is {})", secret_number);
    for attempts_count in 1..(MAX_ATTEMPTS + 1) {
        println!("[attempt {}] What's your guess?", attempts_count);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let user_input_int: i8 = i8::from_str_radix(input.trim(), 10).unwrap_or(-1);
        if user_input_int == -1 {
            println!("This is not a number :-/");
            continue;
        }
        let user_input_uint = user_input_int.abs() as u8;
        let result = check_user_input(secret_number, user_input_uint);

        match result {
            UserInputCheckResult::TooSmall => println!("Too small!"),
            UserInputCheckResult::TooBig => println!("Too Big!"),
            UserInputCheckResult::FoundIt => {
                println!("You found it!!! (in {} attempts)", attempts_count);
                return;
            }
        }
    }
    println!("You haven't found it :-/ (it was {})", secret_number);
}

fn get_secret_number() -> u8 {
    let mut rng_generator = rand::thread_rng();
    return rng_generator.gen_range(MIN, MAX + 1);
}

enum UserInputCheckResult {
    TooSmall,
    TooBig,
    FoundIt,
}

fn check_user_input(secret_number: u8, user_input: u8) -> UserInputCheckResult {
    return if user_input < secret_number {
        UserInputCheckResult::TooSmall
    } else if user_input > secret_number {
        UserInputCheckResult::TooBig
    } else {
        UserInputCheckResult::FoundIt
    };
}
