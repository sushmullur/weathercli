use std::io::{self, Write};
mod weather;

fn main() {
    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command == "exit" || command == "quit" {
            break;
        } 

        process_command(command);
    }
}

fn process_command(command: &str) {
    // Split up command and arguments
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    if command == "weather" {
        weather::process_command(args);
    } else {
        println!("Usage: weather <city> <country>")
    }
}
 