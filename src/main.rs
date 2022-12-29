use rand::Rng;
use std::io;
use std::process;
extern crate clap;
use clap::Parser;

//Create the struct to hold CLI arguments as shown in the clap crate
//https://docs.rs/clap/latest/clap/
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

//Struct to hold the results of dice rolls as well as the type of die used for the rolls
//Using mostly u32 because all die and results should be positive whole numbers
struct Dice {
    results: Vec<u32>,
    die: u32,
}

impl Dice {
    //Function for printing Dice struct in a nice way
    fn print_result(&self) {
        //Simpler output for single die roll, don't need total or count of dice
        if self.results.len() == 1 {
            println!("Result: {}", self.results[0])
        } else {
            //If more than one die, print total and number and type of dice rolled
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

    //Function for deleting the lowest roll, called with -l
    fn lose_lowest(&mut self) -> (usize, u32){
        //Store the lowest value and the index of that value for later printing
        let mut lowest = self.results[0];
        let mut index = 0;

        for i in 1..self.results.len() {
            if self.results[i] < lowest {
                lowest = self.results[i];
                index = i;
            }
        }
        //Return the index and value that were removed for later use
        (index, self.results.remove(index))
    }

    //Function for rerolling die of a given value, called with -r
    fn reroll(&mut self, nums: Vec<u32>) -> Vec<usize> {
        //Storing indexes of matching results to later print which rolls get rerolled
        //This makes it clearer to user which dice are being rerolled
        let mut indexes: Vec<usize> = Vec::new();
        for val in nums {
            for i in 0..self.results.len() {
                if val == self.results[i] {
                    //Printed rolls are not 0 indexed so add 1
                    indexes.push(i + 1);
                    //Dice struct stores self.die so that reroll method can use to roll
                    //new die of same time here
                    self.results[i] = roll(self.die);
                }
            }
        }
        //Sort the indexes so the rolls appear in the order they were originally rolled, 
        //rather than the order that they matched
        indexes.sort();
        indexes
    }
}

//Simple function to roll a single die of given number of sides
fn roll(die: u32) -> u32 {
    rand::thread_rng().gen_range(1..=die)
}

//Function to roll given number of dice of given number of sides, returns a vector of results
//to be stored in Dice::results
fn roll_multiple(die: u32, num: u32) -> Vec<u32> {
    let mut results = vec![];
    for _i in 0..num {
        results.push(roll(die));
    }
    results
}

//Functionality to get an instance of Dice when no arguments are passed
fn no_args() -> Dice {
    //Asks for number of dice to roll, with some error handling for anything that doesn't make sense
    println!("Number of dice to roll: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input.");

    let num: u32 = num.trim().parse().unwrap_or(0);

    if num == 0 {
        println!("Please enter a positive, whole number.");
        process::exit(1);
    }

    //Same as above but with the number of sides per die
    println!("Type of die to roll: ");
    let mut die = String::new();
    io::stdin().read_line(&mut die).expect("Failed to read input.");

    let die: u32 = die.trim().parse().unwrap_or(0);

    if die == 0 {
        println!("Please enter a positive, whole number.");
        process::exit(1);
    }
    
    //Creating and returning the instance of Dice after generating the random results
    Dice {
        results: roll_multiple(die, num),
        die,
    }
}

fn main() {
    //Grab our arguments
    let cli = Cli::parse();
    //Declare result here to maintain ownership and scope for whole main()
    let mut result: Dice;
    
    //As long as there is at least one ordered arg, initializing result is easy with roll_multiple()
    //cli.num defaults to 1, so if called with only 1 arg, will roll 1 die of given type
    if let Some(die) = cli.die {
        result = Dice {
            results: roll_multiple(die, cli.num),
            die,
        }
    } else {
        //If no args then use function defined earlier to ask individually
        result = no_args();
    }

    //Print results using defined method
    result.print_result();

    //Check for rerolls first, because they will often be the lowest
    if let Some(string) = cli.reroll {
        //Collect comma separated list of results to reroll
        let values: Vec<&str> = string.split(',').collect();
        //New vec for u32 after parsing needs to stay in scope
        let mut nums: Vec<u32> = Vec::new();
        for val in values {
            if let Ok(num) = val.parse::<u32>() {
                //Translate &str to u32 for use
                nums.push(num)
            } else {
                //Quit with explanation if invalid arg is given
                println!("{} is not an acceptable number to reroll.", val);
                process::exit(1);
            }
        }
        //Reroll any results that need to be rerolled and store the indexes
        let indexes = result.reroll(nums);
        
        //If nothing was rerolled, print that
        if indexes.is_empty() {
            println!("No rerolls necessary.");
        } else {
            //Here we construct a string listing the indexes of the rolls that were rerolled
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
            //Then those are printed along with the new result
            println!("\nRerolling rolls {}...", rolls);
            println!("New results:");
            result.print_result();
        }
    }

    //Check for lowest flag next because nothing else mutates result
    if cli.lowest {
        let lowest = result.lose_lowest();

        //Print which roll and the value of the roll which was removed
        println!("\nRoll {} was the lowest with value of {}\n", lowest.0 + 1, lowest.1);
        println!("New results:");
        //Then print the new results
        result.print_result();
    }

    //Check for info flag last because it's info on the final result
    if cli.info {
        //To calculate averages (which are often not whole numbers), must convert to floats
        let num = result.results.len() as f64;
        let die = result.die as f64;
        let len = result.results.len() as f64;
        let mut sum: f64 = 0.0;
        let mut high = result.results[0];
        let mut low = result.results[0];

        //Just the general math and printing of results
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
