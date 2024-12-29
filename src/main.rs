use std::io::{self, Write};
use colored::Colorize;
use rand::Rng;
use std::thread;

const WORDLE_WORDS: [&str; 50] = [
    "FLAME",
    "BREAK",
    "STARE",
    "POUND",
    "CHIEF",
    "CLOUD",
    "SMART",
    "BRAVE",
    "GRADE",
    "PILOT",
    "STEAK",
    "DREAM",
    "PLANT",
    "WHOLE",
    "SHINE",
    "BLAST",
    "CRANE",
    "PAINT",
    "SHARP",
    "FLOAT",
    "BREAD",
    "SPACE",
    "TRACE",
    "BRAIN",
    "SWEET",
    "PHASE",
    "PLATE",
    "GHOST",
    "TRAIN",
    "FLAME",
    "WORLD",
    "STAGE",
    "BEACH",
    "CLEAN",
    "HEART",
    "THEME",
    "SHAPE",
    "BLACK",
    "SPARE",
    "MEANT",
    "SCALE",
    "FRAME",
    "THINK",
    "BLOCK",
    "SLATE",
    "PLACE",
    "STEAM",
    "SHARE",
    "PEACE",
    "SPARK"
];

fn main() {
    let random_index = rand::thread_rng().gen_range(0, 50);
    println!("Random wordle word: {}", WORDLE_WORDS[random_index as usize]);
    println!("Guess a word: ");
    let mut tries: i8 = 0;
    while tries <= 4 {
        let mut guess = String::new();


        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        guess = guess.trim().to_string();

        // Erase the user input from the terminal
        clear_previous_line();

        if guess.len() != 5 {
            println!("{}", "5 letter words".red());
            thread::sleep(std::time::Duration::from_millis(500));
            clear_previous_line();
            continue;
        }

        if !guess.chars().all(|c| c.is_alphabetic()) {
            println!("{}", "Only letters allowed".red());
            thread::sleep(std::time::Duration::from_millis(500));
            clear_previous_line();


            continue
        }

        for (i, c) in guess.chars().enumerate() {
            if WORDLE_WORDS[random_index as usize].chars().nth(i).unwrap() == c.to_uppercase().nth(0).unwrap() {
                print!("{}", c.to_string().to_uppercase().green());
            } else {
                print!("{}", c.to_string().to_uppercase().red());
            }
            io::stdout().flush().unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("");

        if guess == WORDLE_WORDS[random_index as usize] {
            println!("You win!");
            return
        }
        else{
            tries+=1;
        }

    }

    println!("You lose!");
}


fn clear_previous_line() {
    print!("\x1B[1A"); // Move the cursor up by one line
    print!("\x1B[2K"); // Clear the entire line
}