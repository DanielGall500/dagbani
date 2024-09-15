pub mod data_reader;
pub mod pw;
use std::{
    collections::HashMap, error::Error, process::exit
};

use pw::PhonologicalWord;

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

fn run() -> Result<(), Box<dyn Error>> {
    let data_path = String::from("data/dagbani-dataset.csv");
    let dataset = data_reader::Dataset::new(data_path)?;
    let data_vec: &Vec<data_reader::Record> = dataset.get_data()?;

    let mut pw_vec: Vec<PhonologicalWord> = Vec::new();
    for word in data_vec {
        let pw = pw::PhonologicalWord::new(word.phonetic_rep.clone());
        pw_vec.push(pw);
    }
    exp_permitted_vowels(&pw_vec);
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