pub mod pw;

fn main() {
    println!("Hello, world!");
    let example = String::from("Hello!");
    let word_example = pw::PhonologicalWord::new(example);
    word_example.get_cv_structure();
}
