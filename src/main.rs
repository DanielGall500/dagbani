pub mod pw;
pub mod data_reader;
use std::{
    process::exit,
    error::Error,
};

fn run() -> Result<(), Box<dyn Error>> {
    let data_path = String::from("data/dagbani-dataset.csv");
    let dataset = data_reader::Dataset::new(data_path)?;
    for word in &dataset.data {
        println!("{}", word.pos);
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
    let example = String::from("Hello!");
    let word_example = pw::PhonologicalWord::new(example);
    word_example.get_cv_structure();

    if let Err(err) = run() {
        println!("{}", err);
        exit(1);
    }

    
}