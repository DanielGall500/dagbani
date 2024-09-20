use unicode_normalization::char::decompose_canonical;

fn remove_diacritics(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        decompose_canonical(ch, |c| {
            if c.is_alphabetic() {
                result.push(c);
            }
        });
    }
    result
}


// defines the phonological word and its functions
pub struct PhonologicalWord {
    pub phon_word: String,
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

    pub fn get_vowels(&self) -> String {
        let decomposed = remove_diacritics(&self.phon_word);
        let vowels = decomposed.chars().filter(
            |c| 
            !self.consonants.contains(c)).collect();
        vowels
    }

    // TODO: consonant cluster impl, e.g CVCCCV 
    pub fn get_cv_structure(&self) -> Vec<&str> {
        let decomposed_word = remove_diacritics(&self.phon_word);

        let mut cv_structure: Vec<&str> = Vec::new();
        let mut is_syllable_closed = true;

        let cv_simple: String = self.get_cv_structure_simple();
        let n_vowels_left: usize = self.get_vowels().chars().count();
        let mut n_vowels_left: i8 = n_vowels_left as i8;

        for (i, phon) in decomposed_word.chars().enumerate() {
            if self.consonants.contains(&phon) {
                if i + 2 <= cv_simple.len() && &cv_simple[i..i+2] == "CV" {
                    cv_structure.push("C_Onset"); 
                    is_syllable_closed = false;
                } else if !is_syllable_closed { // we must close the syllable before moving on
                    cv_structure.push("C_Coda"); 
                    is_syllable_closed = true;
                } else if n_vowels_left >= 1 { // there is another syllable to be parsed
                    cv_structure.push("C_Onset"); 
                    is_syllable_closed = false;
                }
            } 
            else {
                cv_structure.push("V"); 
                is_syllable_closed = false;
                n_vowels_left -= 1;
            }
        }

        cv_structure
    }

    pub fn get_cv_structure_simple(&self) -> String {
        let decomposed_word = remove_diacritics(&self.phon_word);

        let mut cv_structure: String = String::new();
        for phon in decomposed_word.chars() {
            if self.consonants.contains(&phon) {
                cv_structure.push('C');
            }
            else {
                cv_structure.push('V');
            }
        }
        cv_structure
    }

    pub fn get_prosodic_position(&self) -> String {
        let cv_structure: String = self.get_cv_structure_simple();
        if cv_structure == "CV" {
            String::from("Free-Standing CV")
        }
        else if cv_structure == "CVC" {
            String::from("Free-Standing CVC")
        }
        else {
            String::from(">1 Syllable")
        }
    }

}