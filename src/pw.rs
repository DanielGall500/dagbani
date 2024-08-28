// defines the phonological word and its functions
pub struct PhonologicalWord {
    phon_word: String,
    pub consonants: [char; 16],
}

impl PhonologicalWord {
    // constructor to initialize the struct with consonants
    pub fn new(word: String) -> Self {
        PhonologicalWord {
            phon_word: word,
            consonants: ['b', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
                         'm', 'n', 'p', 'r', 's', 't', 'w', 'y'],
        }
    }

    pub fn get_cv_structure(&self) -> String {
        for phon in self.phon_word.chars() {
            println!("Checking {}", phon);
            if self.consonants.contains(&phon) {
                println!("Found consonant.");
            }
            else {
                println!("Found vowel.");
            }
        }
        return String::from("Completed.");
    }

}