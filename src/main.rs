pub mod pw;
pub mod data_reader;
use std::{
    process::exit,
    error::Error,
};

fn run() -> Result<(), Box<dyn Error>> {
    let data_path = String::from("data/dagbani-dataset.csv");
    let dataset = data_reader::Dataset::new(data_path)?;
    let data_vec: &Vec<data_reader::Record> = dataset.get_data()?;
    for word in data_vec {
        let pw = pw::PhonologicalWord::new(word.phonetic_rep.clone());
        let cv_structure = pw.get_cv_structure();

        println!("{}: {}", word.phonetic_rep, cv_structure);
    }
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