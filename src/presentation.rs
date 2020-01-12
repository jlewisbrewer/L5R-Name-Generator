extern crate rand;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use self::rand::random;
use domain::get_file_contents;
pub use classes::name::Name as Name;
pub use classes::clan_name::ClanName as ClanName;

pub fn generate_name<'a>(sex: &str)-> Name {

    // Male initialization
    let male_roots : Vec<Name> = get_file_contents("data/male_names/root.txt");
    let male_prefixes : Vec<Name> = get_file_contents("data/male_names/prefix.txt");
    let male_suffixes : Vec<Name> = get_file_contents("data/male_names/suffix.txt");
    let male_zokumyou : Vec<Name> = get_file_contents("data/male_names/zokumyou.txt");
    let male_yobina : Vec<Name> = get_file_contents("data/male_names/yobina.txt");

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