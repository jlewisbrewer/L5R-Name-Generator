extern crate rand;

use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
// use std::io::Error;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use self::rand::random;

pub use classes::name::Name as Name;
pub use classes::clan_name::ClanName as ClanName;

pub fn get_file_contents(path_name: &str)-> Vec<Name> {
    
    let path  = Path::new(path_name);
    let file = File::open(&path);
    let mut name_vector : Vec<Name> = Vec::new();
    for line in BufReader::new(file.unwrap()).lines() {
        let mut word = Name::new();
        let temp : String = line.unwrap();
        let v: Vec<&str> = temp.split("\t").collect();
        word.kanji = v[0].to_string();
        word.kunyomi = v[1].to_string();
        word.onyomi = v[2].to_string();
        word.english = v[3].to_string();
        name_vector.push(word);
    }
    name_vector
}

pub fn update_file(sex: &str, name_type: &str, info: Name) {
    match sex.to_lowercase().as_ref() {
        "male" => {
            match name_type.to_lowercase().as_ref() {
                "root" => {
                    write_to_file("data/male_names/root.txt", info);                
                }
                "prefix" => {
                    write_to_file("data/male_names/prefix.txt", info);
                }
                "suffix" => {
                    write_to_file("data/male_names/suffix.txt", info);
                }
                "zokumyou" => {
                    write_to_file("data/male_names/zokumyou.txt", info);
                }
                "yobina" => {
                    write_to_file("data/male_names/yobina.txt", info);
                }
                _ => {
                    panic!("incorrect name type");
                }
            };
        }
        "female" => {
            match name_type.to_lowercase().as_ref() {
                "root" => {
                    write_to_file("data/female_names/root.txt", info);                   
                }
                "prefix" => {
                    write_to_file("data/female_names/prefix.txt", info);
                }
                "suffix" => {
                    write_to_file("data/female_names/suffix.txt", info);
                }
                _ => {
                    panic!("incorrect name type");
                }
            };
        }
        "clan" => {
            match name_type.to_lowercase().as_ref() {
                "crab" => {
                    write_to_file("data/clan_names/crab.txt", info);
                }
                "crane" => {
                    write_to_file("data/clan_names/crane.txt", info);
                }
                "dragon" => {
                    write_to_file("data/clan_names/dragon.txt", info);
                }
                "lion" => {
                    write_to_file("data/clan_names/lion.txt", info);
                }
                "phoenix" => {
                    write_to_file("data/clan_names/phoenix.txt", info);
                }
                "scorpion" => {
                    write_to_file("data/clan_names/scorpion.txt", info);
                }
                "unicorn" => {
                    write_to_file("data/clan_names/unicorn.txt", info);
                }
                _=> {
                    panic!("incorrect clan name");
                }
            }
        }
        _ => {
            panic!("incorrect sex");
        }
    };
}
// This function was sourced from stack overflow
// https://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file
// author: Shepmaster, date: 6/6/15
pub fn write_to_file(file_name: &str, input: Name) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    if let Err(e) = writeln!(file, "{}\t{}\t{}\t{}", input.kanji, input.kunyomi, input.onyomi, input.english) {
        eprintln!("Couldn't write line to file: {}", e);
    }
}

// A loop that asks user for information to write to the name files
pub fn add_name_to_file() {
    loop {
        let mut name = String::new();
        let mut input = String::new();
        let mut kanji = String::new();
        let mut kunyomi = String::new();
        let mut onyomi = String::new();
        let mut english = String::new();
        let mut sex = String::new();
        let mut name_type = String::new();
        let mut new_word = Name::new();

        println!("Please select [c]lan name or [p]ersonal name: ");

        match io::stdin().read_line(&mut name) {
            Ok(_n) => {
                match name.trim().as_ref() {
                    "c" => {

                        sex = "clan".to_string();
                        println!("Please enter the clan you wish to add a name to: ");
                        match io::stdin().read_line(&mut input) {
                            Ok(_n) => {
                                match input.trim().to_lowercase().as_ref() {
                                    "crab" => { name_type = "crab".to_string(); }
                                    "crane" => { name_type = "crane".to_string(); }
                                    "dragon" => { name_type = "dragon".to_string(); }
                                    "lion" => { name_type = "lion".to_string(); }
                                    "phoenix" => { name_type = "phoenix".to_string(); }
                                    "scorpion" => { name_type = "scorpion".to_string(); }
                                    "unicorn" => { name_type = "unicorn".to_string(); }
                                    _=> break,
                                };
                            }
                            Err(error) => println!("error: {}", error),
                        }
                    }
                    "p" => {

                        println!("Please enter a gender: [m]ale, [f]emale: ");

                        match io::stdin().read_line(&mut input) {
                            Ok(_n) => {
                                match input.trim().as_ref() {
                                    "m" => {sex = "male".to_string();}
                                    "f" => {sex = "female".to_string();}
                                    _=> break,
                                };
                            }
                            Err(error) => println!("error: {}", error),
                        }
                        println!("Please enter a type: [r]oot, [p]refix, [s]uffix, [z]okumyou, [y]obina suffix");
                        let mut input = String::new();
                        match io::stdin().read_line(&mut input) {
                            Ok(_n) => {
                                match input.trim().as_ref() {
                                    "r" => { name_type = "root".to_string();}
                                    "p" => { name_type = "prefix".to_string();}
                                    "s" => { name_type = "suffix".to_string();}
                                    "z" => { name_type = "zokumyou".to_string();}
                                    "y" => { name_type = "yobina".to_string();}
                                    _ => break,
                                };
                            }
                            Err(error) => println!("error: {}", error),
                        }


                    }
                    _=> break,
                };
            }
            Err(error) => println!("error: {}", error),

        }

        println!("Enter a kanji: ");
        io::stdin().read_line(&mut kanji);
        new_word.kanji = kanji.trim().to_string();
        println!("Enter the kunyomi: ");
        io::stdin().read_line(&mut kunyomi);
        new_word.kunyomi = kunyomi.trim().to_string();
        println!("Enter the onyomi: ");
        io::stdin().read_line(&mut onyomi);
        new_word.onyomi = onyomi.trim().to_string();
        println!("Enter the english: ");
        io::stdin().read_line(&mut english);
        new_word.english = english.trim().to_string();

        update_file(&sex, &name_type, new_word);
        println!("File updated!")

    }
}