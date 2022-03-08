use rand::{thread_rng, Rng};
use std::io::stdin;

/// Get a positive integer from the user
fn read_input() -> u32 {
    println!("Enter how many dice to roll:");
    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        match buffer.trim().parse() {
            Ok(input) => return input,
            Err(_) => println!("Please input a positive integer:"),
        }
    }
}

/// Returns a random number from 1 to 6
fn roll_die() -> u32 {
    thread_rng().gen_range(1..=6)
}

/// Returns an array of length `count` filled with random numbers 1 to 6
fn roll_dice(count: u32) -> Vec<u32> {
    (0..count).map(|_| roll_die()).collect()
}

/// Display the list of rolls to the user, also the sum
fn display_results(rolls: Vec<u32>) {
    for (index, roll) in rolls.iter().enumerate() {
        let index = index + 1;
        println!("#{index}: {roll}");
    }

    let total: u32 = rolls.iter().sum();
    println!("Total: {total}");
}

fn main() {
    let count = read_input();
    let rolls = roll_dice(count);
    display_results(rolls);
}
