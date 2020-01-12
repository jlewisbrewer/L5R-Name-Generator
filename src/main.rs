extern crate rand;
use std::io;

mod presentation;
mod domain;
mod classes;

fn main() {
    println!("L5R Name Generator\n");
    loop {
        println!("\nInput [g] to generate name or [a] to add name. Press any 
        other key to exit.");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().as_ref() {
                    "g" => {
                        presentation::user_generate_name();
                    }
                    "a" => {
                        domain::add_name_to_file();
                    }
                    _ => break,
                };
            }
            Err(error) => println!("error: {}", error),
        };
    }
}