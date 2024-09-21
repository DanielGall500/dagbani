pub mod data_reader;
pub mod pw;
pub mod fst;
pub mod dagbani_fst;
use std::{
    collections::HashMap, error::Error, process::exit, collections::HashSet
};

use pw::PhonologicalWord;
use dagbani_fst::DagbaniFST;

fn exp_permitted_vowels(words: &Vec<PhonologicalWord>) {
    let mut permitted_vowels: HashMap<String, Vec<String>> = HashMap::new();
    for pw in words {
        let cv_structure = pw.get_prosodic_position();
        let vowels = pw.get_vowels();
        println!("{}: {}", pw.phon_word, cv_structure);
        println!("Vowels: {}", vowels);

        permitted_vowels.entry(cv_structure)
        .or_insert(Vec::new()) // Insert a new vector if the key doesn't exist
        .push(vowels); 
    }
    for (key, value) in &permitted_vowels {
        println!("{}: {:?}", key, value);
    }
}

fn exp_permitted_vowels_adv(words: &Vec<PhonologicalWord>) {
    // a hash set ensures that it is a unique set of the permitted vowels
    // rather than storing the same vowels again and again
    let mut permitted_vowels_mora: HashMap<String, HashSet<char>> = HashMap::new();
    let mut permitted_vowels_cv: HashMap<String, HashSet<char>> = HashMap::new();
    let fst: DagbaniFST = DagbaniFST::new();
    for pw in words {
        let cv = pw.get_cv_structure_simple();
        let vowels = pw.get_vowels();
        let vowels_as_vec: Vec<char> = vowels.chars().collect();
        println!("Vowels: {}", vowels);

        match fst.process(&pw) {
            Ok(output) => {
                println!("{}", output);

                // remove irrelevant FST output
                let cleaned: String = output.chars().filter(|c| *c != 'λ').collect();
                
                // Assuming `structure` is defined and you want to use `vowels` here
                for v in vowels_as_vec {
                    // store which vowels appear by mora quantity
                    permitted_vowels_mora.entry(cleaned.clone())
                        .or_insert(HashSet::new()) // Insert a new vector if the key doesn't exist
                        .insert(v); // Push the permitted vowels

                    // store which vowels appear by CV structure
                    permitted_vowels_cv.entry(cv.clone())
                        .or_insert(HashSet::new()) // Insert a new vector if the key doesn't exist
                        .insert(v); // Push the permitted vowels
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            },
        }
    }

    // print out the final results
    for (key, value) in &permitted_vowels_mora {
        let key: String = key.chars().filter(|c| *c != 'λ').collect();
        println!("{}: {:?}", key, value);
    }
    println!("----");
    for (key, value) in &permitted_vowels_cv {
        let key: String = key.chars().filter(|c| *c != 'λ').collect();
        println!("{}: {:?}", key, value);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let data_path = String::from("data/dagbani-dataset.csv");
    let dataset = data_reader::Dataset::new(data_path)?;
    let data_vec: &Vec<data_reader::Record> = dataset.get_data()?;

    let mut pw_vec: Vec<PhonologicalWord> = Vec::new();
    for word in data_vec {
        let pw = pw::PhonologicalWord::new(word.phonetic_rep.clone());
        pw_vec.push(pw);
    }
    // exp_permitted_vowels(&pw_vec);
    exp_permitted_vowels_adv(&pw_vec);
    Ok(())
}

fn main() {
    println!("Hello, world!");
    let example = String::from("píígígɨlɨniɛ́m");
    // let word_example = pw::PhonologicalWord::new(example);
    // word_example.get_cv_structure();

    if let Err(err) = run() {
        println!("{}", err);
        exit(1);
    }
}