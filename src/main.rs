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

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Progress::Continue => write!(f, "Continue"),
            &Progress::End => write!(f, "End")
        }
    }
}

impl GameState {
    fn new(target: &str) -> Self {
        GameState { 
            target: target.to_string(), 
            guesses: Vec::new(),
            attempts_remaining: hangman_terminal::STATES.len() as u8
        }
    }

    fn target_lowercase(&self) -> String {
        self.target.to_ascii_lowercase()
    }

    fn show_man(&self) {
        let index_to_show = hangman_terminal::STATES.len() - self.attempts_remaining as usize;
        match hangman_terminal::STATES.get(index_to_show) {
            Some(man) => println!("{}", man),
            None => println!("{}", hangman_terminal::STATES.last().unwrap())
        }
    }

    fn guess(&mut self, letter: char) -> Result<bool, &str> {
        let letter = letter.to_ascii_lowercase();
        
        if self.guesses.contains(&letter) {
            if self.target.contains(letter){
                return Err("You already guessed that ...");
            } else {
                return Err("You already guessed that incorrectly...");
            }
        }

        let is_correct = self.target_lowercase().contains(letter);
        if !is_correct {
            self.attempts_remaining -= 1
        }

        self.guesses.push(letter);

        return Ok(is_correct);
    }
    
    /// Prints the man, then 
    /// the incorrect guesses, then
    /// the progress on the word, then
    /// the number of attempts remaining
    
    fn render(&mut self) -> Progress {
        /* First, show the man */
        self.show_man();

        /* Second, show all their incorrectly guessed letters */
        self.guesses.sort();

        let incorrect_guesses: String = self.guesses.iter().map(|c: &char| {
            if !self.target_lowercase().contains(c.clone()) {
                format!("{} ", c)
            } else {
                String::from("")
            }
        }).collect();
        
        println!("{incorrect_guesses}");

        /* Third, format the word and show attempts remaining */
        let word: String = self.target_lowercase().chars().map(|c: char| {
            if (self.guesses.contains(&c)) || (c == ' ') {
                format!("{c} ")
            } else {
                String::from("_ ")
            }
        }).collect();
        
        println!("{}\n{} attempts remaining\n", word, self.attempts_remaining - 1);

        /* Fourth, return status */
        if self.attempts_remaining == 1 {
            Progress::End
        } else if word.contains('_') {
            Progress::Continue
        } else {
            Progress::End
        }
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
    execute!(
        io::stdout(), 
        Clear(ClearType::All), 
        crossterm::cursor::MoveTo(0, 0)
    ).unwrap();
}

fn play() {
    let target = ascii_input("Please enter a phrase for the other player to guess.");

    let mut hangman = GameState::new(&target);
    
    clear_terminal();
    loop {
        let state = hangman.render();
        if let Progress::End = state {
            break
        }

        let answer =  ascii_input("Guess a letter.");
        clear_terminal();
        
        if answer.len() != 1 {
            println!("Please enter just a single letter")
        } else {
            let result = hangman.guess(answer.chars().nth(0).unwrap());
            match result {
                Ok(true) => println!("Correct guess!"),
                Ok(false) => println!("Incorrect guess :("),
                Err(msg) => println!("{msg}")
            }
        }
    }

    println!("Game over!")
}

fn main() {
    println!("Welcome to Rust Hangman 0.6.1!");
    
    loop {
        play();
        let encore: bool = loop {
            let response = ascii_input("Do you want to play again?");
            match response.as_str() {
                "y" | "Y" | "yes" | "Yes" => break true,
                "n" | "N" | "no" | "No" => break false,
                _ => println!("🤦\nPlease enter y/n")
            }
        };

        if !encore {
            break;
        }
    }
}
