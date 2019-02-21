mod user_presentation {
// A loop that asks user for information to write to the name files
fn add_name_to_file() {
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

}