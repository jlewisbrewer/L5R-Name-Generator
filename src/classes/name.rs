// extern crate rand;
// use rand::random;
pub struct Name {
    pub kanji: String,
    pub onyomi: String,
    pub kunyomi: String,
    pub english: String,
}

impl Name {
    pub fn new() -> Name {
        Name {kanji: "".to_string(),
            kunyomi: "".to_string(),
            onyomi: "".to_string(),
            english: "".to_string()}
    }
    pub fn display(&self) {
        print!("\tKanji: {}\n\tKunyomi: {}\n\tOnyomi: {}\n\tEnglish: {}\n", 
                &self.kanji, &self.kunyomi, &self.onyomi, &self.english);
    }
}