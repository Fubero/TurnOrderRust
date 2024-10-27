use std::io;
use std::fs;

fn main() {
    let logo = fs::read_to_string("./assets/logo.txt").expect("Can't open the impressiv logo...");
    println!("{logo}");

    loop {
        println!("What is your command, Master!");
        
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read input!");
        command = command.trim().to_string();

        println!("Your command was {command}");

        if command == "end"{
            break;
        }
    }
}
