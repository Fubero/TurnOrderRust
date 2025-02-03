use std::default;
use std::io;
use std::fs;
use colored::Colorize;

fn main() {
    let logo = fs::read_to_string("./assets/logo.txt").expect("Can't open the impressive logo...");
    println!("{}",logo.green());

    let mut turn_order: Vec<Character> = Vec::new();

    loop {
        println!("What is your command, Mylord?");
        
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read input!");
        command = command.trim().to_string();

        //println!("Your command was '{command}'");

        match command.as_str(){
            "end" => break,
            "load"=> {
                println!("not implemented at the moment");
                println!("");
            },
            "add"=> {
                add(&mut turn_order);
                println!("");
            },
            "show"=> {
                show(&turn_order);
                println!("");
            }
            _=>println!("This Command didn't exist!\n"),
        }
        println!("");
    }
}

fn add(list: &mut Vec<Character>) {
    loop {
        
    
    let mut new_player: Character = Character::new();
    
    println!("Player Name?");
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input!");
    n = n.trim().to_string();
    if n == "end"{
        break;
    }
    new_player.player_name = n;
    println!("");

    println!("Character Name?");
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input!");
    new_player.character_name = n.trim().to_string();
    println!("");

    println!("Die?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input!");
    let n: i8 = n.trim().parse().expect("invalid input");
    new_player.die = n;
    println!("");

    println!("Modifier?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input!");
    let n: i8 = n.trim().parse().expect("invalid input");
    new_player.modifier = n;
    println!("");

    new_player.initiative = new_player.die + new_player.modifier;

    list.push(new_player);

    println!(">> Next Player (type 'end' to escape)");
    println!();
    }
}

fn show (list: &Vec<Character>){
    for player in list{
        player.display();
    }
}


pub struct Character {
    player_name: String,
    character_name: String,
    die: i8,
    modifier: i8,
    initiative: i8,
}
impl Character {
    fn new() -> Self {
        Character {
            player_name: "".to_string(),
            character_name:"".to_string(),
            die: 0,
            modifier: 0,
            initiative: 0
        }
    }
}

impl Character {
    pub fn display(&self) {
        println!("|#################################");
        println!("|# Player Name: {}",self.player_name);
        println!("|#--------------------------------");
        println!("|# Character: {}",self.character_name);
        println!("|# Initiative: {}",self.initiative);
        println!("|#################################");
        println!("");
    }
}