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
        let decomposed_word = remove_diacritics(&self.phon_word);
        println!("Word: {}", decomposed_word);

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

}