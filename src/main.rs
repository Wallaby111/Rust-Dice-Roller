use rand::Rng;
use std::io;
use std::process;
use std::env;

fn help_menu() {
    println!("This is a command line utility to quickly roll multiple (or single) dice and sum their values.");
    println!("It has been designed mostly for use with role playing games like D&D, but can be used any time dice are used.");
    println!("This was originally written in Go, but I wanted to rewrite and hopefully improve it in Rust.");
    println!("\nUsage:\n");
    println!("Can be run with no arguments, 1 argument or 2 arguments.");
    println!("With no arguments, you will be asked to input the number of dice you with to roll and then the number of sides you want the die to have.\n");
    println!("With 1 argument, only a single die will be rolled with the number of sides specified in the argument. \n`roll <type of die>`\n");
    println!("With 2 arguments, the first will be the number of dice to roll and the second the number of sides per die. \n`roll <number of dice> <type of die>\n");
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

fn print_result(vec: Vec<u32>) {
    if vec.len() == 1 {
        println!("Result: {}", vec[0])
    } else {
        for i in 0..vec.len() {
            println!("Result {}: {}", i + 1, vec[i]);
        }
        let mut sum: u32 = 0;
        for i in 0..vec.len() {
            sum += vec[i];
        }
        println!("Total of {} die: {}", vec.len(), sum);
    }
}

fn no_args() -> Vec<u32> {
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
    
    roll_multiple(die, num)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_result(no_args())
    } else if args.len() == 2 {
        if args[1] == String::from("help"){
            help_menu();
            process::exit(1);
        }

        let die: u32;
        if let Ok(n) = args[1].trim().parse() {
            die = n;
        } else {
            println!("Usage: roll <type of die>\nArgument is optional and must be a positive, whole integer.\nUse: `roll help` for usage.");
            process::exit(1);
        }
        if die == 0 {
            println!("Cannot roll a 0 sided die.");
        } else {
            let res = roll(die);
            println!("Result: {}", res);
        }
    } else if args.len() == 3 {
        let die: u32;
        let num: u32;

        if let Ok(n) = args[2].trim().parse() {
            die = n;
        } else {
            println!("Usage: roll <number of dice> <type of die>\nArguments are optional and must be whole positive integers.\nUse: `roll help` for usage.");
            process::exit(1);
        }

        if let Ok(n) = args[1].trim().parse() {
            num = n;
        } else {
            println!("Usage: roll <number of dice> <type of die>\nArguments are optional and must be whole positive integers.\nUse: `roll help` for usage.");
            process::exit(1);
        }
        if num == 0 || die == 0 {
            println!("Cannot roll 0 dice or a 0 sided die.");
            process::exit(1);
        }
        if num > 2 {
            println!("Rolling {} D{}s", num, die);
        } else {
            println!("Rolling 1 D{}", die);
        }
        print_result(roll_multiple(die, num));
    } else {
        println!("Usage: roll <type of die> <number of die>\nArguments are optional and must be whole positive integers.\nUse: `roll help` for usage.");
    }
}