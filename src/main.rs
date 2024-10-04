use std::io::{stdin, Result};
use std::result;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() -> Result<()> {
    println!("Welcome to Rustulator!");
    let mut first_in = String::new();
    let mut second_in = String::new();
    let mut operator = String::new();

    loop {
        get_inputs(&mut first_in, &mut second_in, &mut operator)?;

        let first_num = match first_in.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
        };
        let second_num = match second_in.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
        };

        let op = match parse_operation(&operator) {
            Ok(op) => op,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        match calculate(first_num, second_num, op) {
            Some(result) => {
                println!("The result is: {}", result);
                match restart() {
                    Ok(true) => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
                    Ok(false) => {
                        println!("Cya!");
                        break;
                    }
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                }
            }
            None => println!("Error: Division by zero"),
        }
    }
    Ok(())
}

fn get_inputs(first_in: &mut String, second_in: &mut String, op: &mut String) -> Result<()> {
    first_in.clear();
    second_in.clear();
    op.clear();

    println!("Enter first input: ");
    stdin().read_line(first_in)?;

    println!("Enter second input: ");
    stdin().read_line(second_in)?;

    println!("Enter the operator: ");
    stdin().read_line(op)?;
    Ok(())
}

fn parse_operation(op: &str) -> result::Result<Operation, &'static str> {
    match op.trim() {
        "+" => Ok(Operation::Add),
        "-" => Ok(Operation::Subtract),
        "*" => Ok(Operation::Multiply),
        "/" => Ok(Operation::Divide),
        _ => Err("Invalid operation"),
    }
}

fn calculate(first_num: i32, second_num: i32, op: Operation) -> Option<i32> {
    match op {
        Operation::Add => Some(first_num + second_num),
        Operation::Subtract => Some(first_num - second_num),
        Operation::Multiply => Some(first_num * second_num),
        Operation::Divide => {
            if second_num != 0 {
                Some(first_num / second_num)
            } else {
                None // Handle division by zero
            }
        }
    }
}

fn restart() -> Result<bool> {
    println!("Again? (Y/y, n)");
    let mut accept = String::new();
    stdin().read_line(&mut accept)?;

    match accept.trim() {
        "Y" | "y" => Ok(true),
        _ => Ok(false),
    }
}
