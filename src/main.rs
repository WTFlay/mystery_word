extern crate rand;

use std::io;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use rand::Rng;

fn pick_word(dict: &Path) -> String {
    let mut buf = String::new();
    let display = dict.display();
    let mut file = match File::open(&dict) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {} : {}", display, why.description()),
    };
    match file.read_to_string(&mut buf) {
        Ok(_) => {},
        Err(why) => panic!("couldn't read {} : {}", display, why.description()),
    };
    let words: Vec<&str> = buf.split('\n').collect();

    let random_num = rand::thread_rng().gen_range(0, words.len()-1);
    words[random_num].to_string().to_uppercase()
}

fn hash_word(word: &mut String) {
    let mut nb_cara = word.len();
    let mut word_hash = String::new();

    for _ in 0..word.len() {
        let pos = rand::thread_rng().gen_range(0, nb_cara);
        word_hash.push(word.remove(pos));
        nb_cara -= 1;
    }
    word.push_str(&word_hash);
}

fn main() {
    let path_dict = Path::new("/usr/share/dict/words");

    let word_mystery = pick_word(&path_dict);
    let mut word_hash = word_mystery.clone();
    hash_word(&mut word_hash);

    let mut input_word = String::new();
    while input_word != word_mystery {
        println!("What is this word ? {}", word_hash);

        io::stdin().read_line(&mut input_word).expect("error in input");
        input_word.pop();

        match word_mystery.cmp(&input_word) {
            Ordering::Equal => println!("Good ! It was {}", word_mystery),
            _   => println!("{} is not the good word !\n", input_word),
        }
    }
}
