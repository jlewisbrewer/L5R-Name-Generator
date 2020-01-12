extern crate rand;

use super::name::Name;
use rand::random;

pub struct ClanName {
    pub clan: String,
    pub surname: Name,
    pub name: Name,
}

impl ClanName {
    pub fn new(clan: Option<&str>) -> ClanName {
    
        let clan_name: String;
        clan_name = match clan {
            Some(input) => {
                let name = input.to_lowercase();
                match name.as_ref() {
                    "crane" => "Crane".to_string(),
                    "crab" => "Crab".to_string(),
                    "dragon" => "Dragon".to_string(),
                    "lion" => "Lion".to_string(),
                    "phoenix" => "Phoenix".to_string(),
                    "scorpion" => "Scorpion".to_string(),
                    "unicorn" => "Unicorn".to_string(),
                    _ => generate_clan_name(),
                }
            }
            None => generate_clan_name(),
        };
        
        fn generate_clan_name() -> String {
            // let mut rng = rand::thread_rng();
            let mut rand_num : usize = random();
            rand_num = rand_num % 7;
            let result = match rand_num {
                0 => "Crane".to_string(),
                1 => "Lion".to_string(),
                2 => "Dragon".to_string(),
                3 => "Phoenix".to_string(),
                4 => "Unicorn".to_string(),
                5 => "Scorpion".to_string(),
                _ => "Crab".to_string(),
            };
            result
        }
        
        ClanName {
            clan: clan_name,
            surname: Name::new(),
            name: Name::new(),
        }
    }

    pub fn display(&self) {
        print!("\tKanji: {} {}\n\tKunyomi: {} {}\n\tOnyomi: {} {}\n\tEnglish: {} {}\n",
                &self.surname.kanji, &self.name.kanji, &self.surname.kunyomi, &self.name.kunyomi,
                &self.surname.onyomi, &self.name.onyomi, &self.surname.english, &self.name.english);
    }
}
