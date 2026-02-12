mod hangman_terminal;
use std::io::{self, Write};
use crossterm::{execute, terminal::{Clear, ClearType}};

struct GameState {
    target: String,
    guesses: Vec<char>,
    attempts_remaining: u8
}

enum Progress {
    Continue,
    End
}

// Logic
impl GameState {
    fn new(target: &str) -> Self {
        GameState { target: target.to_string(), guesses: Vec::new(), attempts_remaining: 8, }
    }

    fn target_lowercase(&self) -> String {
        self.target.to_ascii_lowercase()
    }

    // will only ever push lowercase letters to self.guesses
    fn guess(&mut self, letter: char) -> Result<bool, &str> {
        let letter = letter.to_ascii_lowercase();
        
        if self.guesses.contains(&letter) {
            return Err("You already guessed that incorrectly...");
        }

        let is_correct = self.target_lowercase().contains(letter);
        if !is_correct {
            self.attempts_remaining -= 1
        }

        self.guesses.push(letter);

        return Ok(is_correct);
        
    }

    fn show(&mut self) -> Progress {
        /* First, show the man */
        self.show_man();

        /* Second, show all their incorrectly guessed letters */
        self.guesses.sort();

        let formatted_guesses: String = self.guesses.iter().map(|c: &char| {
            if !self.target_lowercase().contains(c.clone()) {
                format!("{} ", c)
            } else {
                String::from("")
            }
        }).collect();
        
        println!("{}", formatted_guesses);

        /* Third, remark attempts remaining, or end. */
        let word: String = self.target_lowercase().chars().map(|c: char| {
            if self.guesses.contains(&c) {
                format!("{c} ")
            } else {
                String::from("_ ")
            }
        }).collect();
        
        if word.contains('_') {
            println!("{}\n{} attempts remaining\n", word, self.attempts_remaining);
            Progress::Continue
        } else {
            Progress::End
        }
    }

    fn show_man(&self) {
        let index_to_show = hangman_terminal::STATES.len() - self.attempts_remaining as usize;
        println!("{}", hangman_terminal::STATES[index_to_show]);
    }
}

// Will only ever return ASCII characters.
fn ascii_input(prompt: &str) -> String{
    println!("{prompt}");
    let mut buffer = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut buffer)
            .expect("AAA can't read from standard input! Exiting!!!");
        let answer = String::from(buffer.trim());
        if !answer.is_ascii() {
            println!("This is a simple program and only accepts ascii characters.");
            println!("Please try again.");
        } else {
            break answer
        }
    }
}

fn clear_terminal() {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
}

fn main() {
    println!("Welcome to Rust Hangman 1.0.0!");
    let target = ascii_input("Please enter a phrase for the other player to guess.");

    let mut hangman = GameState::new(&target);
    
    clear_terminal();
    loop {
        let state = hangman.show();
        if let Progress::End = state {
            break
        }

        let answer =  ascii_input("Guess a letter.");
        clear_terminal();
        
        if answer.len() != 1 {
            println!("Please enter just a single letter")
        } else {
            let result = hangman.guess(answer.chars().nth(0).unwrap().to_ascii_lowercase());
            match result {
                Ok(true) => println!("Correct guess!"),
                Ok(false) => println!("Incorrect guess :("),
                Err(msg) => println!("{msg}")
            }
        }
    }

    hangman.show_man();
    println!("Game over!")
}
