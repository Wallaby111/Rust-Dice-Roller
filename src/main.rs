use rand::Rng;
use std::io;
use std::process;
extern crate clap;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Rusty-Dice")]
#[command(author = "Morgan Hester <morganhasebehester@gmail.com>")]
#[command(about = "CLI dice roller", long_about = None)]
struct Cli {
    ///Optional argument for the number of sides each die has
    die: Option<u32>,
    #[arg(default_value_t = 1)]
    ///Optional argument for the number of dice to roll
    num: u32,
}

struct Dice {
    results: Vec<u32>,
    die: u32,
}

impl Dice {
    fn print_result(&self) {
        if self.results.len() == 1 {
            println!("Result: {}", self.results[0])
        } else {
            for i in 0..self.results.len() {
                println!("Result {}: {}", i + 1, self.results[i]);
            }
            let mut sum: u32 = 0;
            for i in 0..self.results.len() {
                sum += self.results[i];
            }
            println!("Total of {} D{}: {}", self.results.len(), self.die, sum);
        }
    }
}

fn roll(die: u32) -> u32 {
    rand::thread_rng().gen_range(1..=die)
}

fn roll_multiple(die: u32, num: u32) -> Vec<u32> {
    let mut results = vec![];
    for _i in 0..num {
        results.push(roll(die));
    }
    results
}

fn no_args() -> Dice {
    println!("Number of dice to roll: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input.");

    let num: u32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    if num == 0 {
        println!("Please enter a positive, whole number.");
        process::exit(1);
    }

    println!("Type of die to roll: ");
    let mut die = String::new();
    io::stdin().read_line(&mut die).expect("Failed to read input.");

    let die: u32 = match die.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    if die == 0 {
        println!("Please enter a positive, whole number.");
        process::exit(1);
    }
    
    Dice {
        results: roll_multiple(die, num),
        die: die,
    }
}

fn main() {
    let cli = Cli::parse();
    let result: Dice;
    
    if let Some(die) = cli.die {
        result = Dice {
            results: roll_multiple(die, cli.num),
            die: die,
        }
    } else {
        result = no_args();
    }

    result.print_result();
}
