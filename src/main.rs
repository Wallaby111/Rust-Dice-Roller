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

    ///Show maximum possible roll, highest roll, lowest roll, expected average roll, and actual average roll
    #[arg(short,long)]
    info: bool,

    ///Get rid of single lowest roll
    #[arg(short,long)]
    lowest: bool,

    ///Reroll any dice of given value (multiple inputs are okay with comma separation)
    #[arg(short,long)]
    reroll: Option<String>
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

    fn lose_lowest(&mut self) -> (usize, u32){
        let mut lowest = self.results[0];
        let mut index = 0;

        for i in 1..self.results.len() {
            if self.results[i] < lowest {
                lowest = self.results[i];
                index = i;
            }
        }
        (index, self.results.remove(index))
    }

    fn reroll(&mut self, nums: Vec<u32>) -> Vec<usize> {
        let mut indexes: Vec<usize> = Vec::new();
        for val in nums {
            for i in 0..self.results.len() {
                if val == self.results[i] {
                    indexes.push(i + 1);
                    self.results[i] = roll(self.die);
                }
            }
        }
        indexes.sort();
        return indexes
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
    let mut result: Dice;
    
    if let Some(die) = cli.die {
        result = Dice {
            results: roll_multiple(die, cli.num),
            die: die,
        }
    } else {
        result = no_args();
    }

    result.print_result();

    if let Some(string) = cli.reroll {
        let values: Vec<&str> = string.split(",").collect();
        let mut nums: Vec<u32> = Vec::new();
        for i in 0..values.len() {
            if let Ok(num) = values[i].parse::<u32>() {
                nums.push(num)
            } else {
                println!("{} is not an acceptable number to reroll.", values[i]);
                process::exit(1);
            }
        }
        let indexes = result.reroll(nums);
        
        if indexes.len() == 0 {
            println!("No rerolls necessary.");
        } else {
            let mut rolls = String::new();
            for i in 0..indexes.len() {
                if i == 0 {
                    rolls.push_str(&indexes[i].to_string());
                } else if i < indexes.len() -1 {
                    rolls.push_str(", ");
                    rolls.push_str(&indexes[i].to_string());
                } else {
                    rolls.push_str(", and ");
                    rolls.push_str(&indexes[i].to_string());
                }
            }
            println!("\nRerolling rolls {}...", rolls);
            println!("New results:");
            result.print_result();
        }
    }

    if cli.lowest == true {
        let lowest = result.lose_lowest();

        println!("\nRoll {} was the lowest with value of {}\n", lowest.0 + 1, lowest.1);
        println!("New results:");
        result.print_result();
    }

    if cli.info == true {
        let num = result.results.len() as f64;
        let die = result.die as f64;
        let len = result.results.len() as f64;
        let mut sum: f64 = 0.0;
        let mut high = result.results[0];
        let mut low = result.results[0];

        for i in 0..result.results.len() {
            sum += result.results[i] as f64;

            if result.results[i] > high {
                high = result.results[i];
            }
            if result.results[i] < low {
                low = result.results[i];
            }
        }

        println!("\nMaximum possible roll: {}", len * die);
        println!("Highest roll: {}", high);
        println!("Lowest roll: {}", low);
        println!("Expected average: {}", (die + 1.0) / 2.0);
        println!("Actual average: {}", sum / num)
    }
}
