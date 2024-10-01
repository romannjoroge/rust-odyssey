use std::{io, process::exit};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Tell the user to guess a number between 0 and 100
    println!("This is the guessing game! When the computer guesses your number you lose! See how many tries it takes for the computer to guess your number!");
    println!("Enter a number between 0 and 100. The number cannot be 0 but can be 100: ");

    let mut guess = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut guess).expect("Failed To Get User Input");

    let mut computer_guess = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;
    let user_guess: u32 = guess.trim().parse().expect("Please enter a valid number");

    if user_guess <= 0 || user_guess > 100 {
        println!("Invalid Number");
        exit(2);
    }

    let mut high = 100;
    let mut low = 0;

    println!("Initial computer guess is {computer_guess} and user's guess is {user_guess}");
    loop {
        match user_guess.cmp(&computer_guess) {
            Ordering::Less  => {
                high = computer_guess;
                let difference = high - low;
                computer_guess = computer_guess - (difference / 2);
                guesses = guesses + 1;
                println!("Computer Guess was greater than user guess. New computer guess is {computer_guess}, new high is {high} and new low is {low}")
            }

            Ordering::Greater => {
                low = computer_guess;
                let difference = high - low;
                computer_guess = computer_guess + (difference / 2);
                guesses = guesses + 1;
                println!("Computer Guess was less than user guess. New computer guess is {computer_guess}, new high is {high} and new low is {low}")
            }

            Ordering::Equal => {
                println!("Your Loss Was Inevitable! Take solace in the fact that it took me {guesses} tries to get it!");
                break;
            }
        }
    }
}
