extern crate rand;

use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
// use std::io::Error;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use self::rand::random;

// use super::*;
// mod classes;
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

pub fn generate_name<'a>(sex: &str)-> Name {

    // Male initialization
    let male_roots : Vec<Name> = get_file_contents("data/male_names/root.txt");
    let male_prefixes : Vec<Name> = get_file_contents("data/male_names/prefix.txt");
    let male_suffixes : Vec<Name> = get_file_contents("data/male_names/suffix.txt");
    let male_zokumyou : Vec<Name> = get_file_contents("data/male_names/zokumyou.txt");
    let male_yobina : Vec<Name> = get_file_contents("data/male_names/yobina.txt");
                    // let mut temp_name = Name::new();

    // Female initialization
    let female_roots: Vec<Name> = get_file_contents("data/female_names/root.txt");
    let female_prefixes : Vec<Name> = get_file_contents("data/female_names/prefix.txt");
    let female_suffixes: Vec<Name> = get_file_contents("data/female_names/suffix.txt");
    
    let mut name = Name::new();
    match sex.to_lowercase().as_ref() {
        "male" => {
            //Randomizes male names
            let rand_num : usize = random();
            let name_type = rand_num % 6;

            let mut prefix : Name;
            let mut suffix : Name;

            match name_type {
                // Name is two roots
                0 => {
                    prefix = get_random_name(&male_roots);
                    suffix = get_random_name(&male_roots);
                }
                // Name is root + suffix
                1 => {
                    prefix = get_random_name(&male_roots);
                    suffix = get_random_name(&male_suffixes);
                }
                // Name is zokumyou + yobina_suffix
                2 => {
                    prefix = get_random_name(&male_zokumyou);
                    suffix = get_random_name(&male_yobina);
                }
                // Name is prefix and zokumyou
                3 => {
                    prefix = get_random_name(&male_prefixes);
                    suffix = get_random_name(&male_zokumyou);
                }
                // Name is prefix and zokumyou and yobina_suffix
                4 => {
                    let mut temp_name = Name::new();
                    let mut temp_zokumyou = Name::new();
                    temp_name = get_random_name(&male_prefixes);
                    temp_zokumyou = get_random_name(&male_zokumyou);
                    prefix = Name::new();
                    prefix.kanji = format!("{}{}", temp_name.kanji, temp_zokumyou.kanji.to_lowercase());
                    prefix.kunyomi = format!("{}{}", temp_name.kunyomi, temp_zokumyou.kunyomi.to_lowercase());
                    prefix.onyomi = format!("{}{}", temp_name.onyomi, temp_zokumyou.onyomi.to_lowercase());
                    prefix.english = format!("{} {}", temp_name.english, temp_zokumyou.english);
                    suffix = get_random_name(&male_yobina);
                }
                // Name is prefix + root
                _ => {
                    prefix = get_random_name(&male_prefixes);
                    suffix = get_random_name(&male_roots);
                }
            };
            name.kanji = format!("{}{}", prefix.kanji, suffix.kanji.to_lowercase());
            name.kunyomi = format!("{}{}", prefix.kunyomi, suffix.kunyomi.to_lowercase());
            name.onyomi = format!("{}{}", prefix.onyomi, suffix.onyomi.to_lowercase());
            name.english = format!("{} {}", prefix.english, suffix.english);
        }
        _ => {
            //Randomizes female names
            let rand_num : usize = random();
            let name_type = rand_num % 4;
            
            let mut prefix : Name;
            let mut suffix : Name;
            match name_type {
                // Name is just root
                0 => {
                    prefix = get_random_name(&female_roots);
                    suffix = Name::new();
                }
                // Name is root + suffix
                1...2 => {
                    prefix = get_random_name(&female_roots);
                    suffix = get_random_name(&female_suffixes);
                    
                }
                // Name is prefix + root
                _ => {
                    prefix = get_random_name(&female_prefixes);
                    suffix = get_random_name(&female_roots);
                    
                }
            };
            name.kanji = format!("{}{}", prefix.kanji, suffix.kanji);
            name.kunyomi = format!("{}{}", prefix.kunyomi, suffix.kunyomi);
            name.onyomi = format!("{}{}", prefix.onyomi, suffix.onyomi);
            name.english = format!("{} {}", prefix.english, suffix.english);
            
        }
    };
    name
    
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

fn get_random_name(array : &Vec<Name>) -> Name {
    let mut name = Name::new();
    let rand_number : usize = random();
    let rand_index = rand_number % array.len();

    name.kanji = array[rand_index].kanji.to_string();
    name.kunyomi = array[rand_index].kunyomi.to_string();
    name.onyomi = array[rand_index].onyomi.to_string();
    name.english = array[rand_index].english.to_string();

    name
}

pub fn user_generate_name() {
    let mut clan = String::new();
    let mut sex = String::new();
    let mut input = String::new();
    let mut clan_name = None;
    let mut number : usize = 5;


    println!("Choose a clan:
    [c]rab
    cra[n]e
    [d]ragon
    [l]ion
    [p]hoenix
    [s]corpion
    [u]nicorn
    [r]andom");
    match io::stdin().read_line(&mut clan) {
        Ok(_)=>{
            match clan.trim().as_ref() {
                "c" => {clan_name = Some("crab");}
                "n" => {clan_name = Some("crane");}
                "d" => {clan_name = Some("dragon");}
                "l" => {clan_name = Some("lion");}
                "p" => {clan_name = Some("phoenix");}
                "s" => {clan_name = Some("scorpion");}
                "u" => {clan_name = Some("unicorn");}
                "r" => {clan_name = None;}
                _ => {panic!("unable to read input");}
            };
        }
        Err(error) => println!("error: {}", error),
    };

    let mut name = ClanName::new(clan_name);
    name.surname = choose_clan_name(&name.clan.to_lowercase());

    println!("How many names do you want to generate? :");
    match io::stdin().read_line(&mut input) {
        Ok(_) => { number = input.trim().parse::<usize>().unwrap();}
        Err(error) => println!("error: {}", error),
    };
                        
    println!("[m]ale or [f]emale names: ");
    match io::stdin().read_line(&mut sex) {
        Ok(_) => {
            match sex.trim().as_ref() {
                "m" => {
                    for x in 0..number {
                        name.name = generate_name("male");
                        println!("Name {} ----------", x + 1);
                        name.display();
                    }
                }
                "f" => {
                    for x in 0..number {
                        name.name = generate_name("female");
                        println!("Name {} ----------", x + 1);
                        name.display();
                    }
                }
                _ => panic!("incorrect sex"),
            };
        }
        Err(error) => println!("error: {}", error),
    };

}

pub fn choose_clan_name(clan: &str) -> Name {
    
    let mut input = String::new();
    let file_name = format!("data/clan_names/{}.txt", clan);
    let clan_names : Vec<Name> = get_file_contents(&file_name);
    let mut result = Name::new();

    println!("Choose a clan name : ");
    for i in 0..clan_names.len() {
        println!("\t{} -- {}", i + 1, clan_names[i].kunyomi);
        
    }
    println!("\tr -- random name");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().as_ref() {
                "r" => result = get_random_name(&clan_names),
                _=> {
                    let mut index = input.trim().parse::<usize>().unwrap();
                    index = index - 1;
                    // assert!(index <= clan_names.len() && index >= 0);

                    // let result = clan_names.remove(index);

                    result.kanji = clan_names[index].kanji.to_string();
                    result.kunyomi = clan_names[index].kunyomi.to_string();
                    result.onyomi = clan_names[index].onyomi.to_string();
                    result.english = clan_names[index].english.to_string();
                },
            };
        }
        Err(error) => println!("error: {}", error),
    };
    result.display();
    result

}
