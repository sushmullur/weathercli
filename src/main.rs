extern crate dotenv;
use std::io::{self, Write};
use dotenv::dotenv;
mod weather;
mod help;
mod convert;

#[tokio::main] 
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    println!("Usage: weather <city> <mode>");
    println!("Enter \"help\" for more information regarding modes.");
    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command == "exit" || command == "quit" {
            break;
        } 

        process_command(command).await;
    }
}

async fn process_command(command: &str) {
    // Split up command and arguments
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    if command == "weather" {
        match weather::process_command(args).await {
            Ok(_) => println!(""),
            Err(err) => println!("Error: {}", err),
        }
    } else if command == "help" {
        help::process_command();
    } else if command == "convert" {
        convert::process_command(args);
    }
    else {
        println!("Usage: weather <city> <mode>")
    }
}
 