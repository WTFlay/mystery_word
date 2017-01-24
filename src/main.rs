extern crate rand;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

use rand::distributions::{IndependentSample, Range};

fn pick_word(dict: &Path) -> &str {
    let mut buf = String::new();
    let display = dict.display();
    let mut file = match File::open(&dict) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {} : {}", display, why.description()),
    };
    match file.read_to_string(&mut buf) {
        Ok(size) => println!("{} bytes read.", size),
        Err(why) => panic!("couldn't read {} : {}", display, why.description()),
    };
    let words: Vec<&str> = buf.split('\n').collect();

    let range_random = Range::new(0, words.capacity());
    let mut rng = rand::thread_rng();
    let random_num = range_random.ind_sample(&mut rng);

    println!("{}", words[random_num]);
    "hello"
}

fn hash_word() {
    
}

fn main() {
    let path_dict = Path::new("/usr/share/dict/words");

    println!("pick_word: {}", pick_word(&path_dict));
    hash_word();

    // While hash_word != user_word
    // Input user word
    // Diff hash_word and user_word
}
