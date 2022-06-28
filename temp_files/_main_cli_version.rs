use gui::gui;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use difflib::get_close_matches as gcm;
use serde_json::{self, Map, Value};

fn read_from_file<P: AsRef<Path>>(path: P) -> Map<String, Value> {
    let mut file = File::open(path).expect("failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read to string");
    // dbg!(&contents);
    let map: Map<String, Value> =
        serde_json::from_str(&contents).expect("failed to parse into Value");
    map
}

fn get_search_term() -> String {
    println!("Enter search term, or 'IQuit' to exit : ");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("error reading input");
    let term = buffer.to_lowercase();
    term
}

fn main() {
    let map = read_from_file("src/jsonformatter.txt");
    let all_word_keys: Vec<&str> = map.keys().map(|s| s.as_ref()).collect();

    loop {
        let search_word = get_search_term();
        println!("{search_word}");
        if search_word.contains("iquit") {
            break;
        }
        let results = gcm(search_word.trim(), all_word_keys.clone(), 3, 0.6);
        if results.is_empty() {
            println!("term not found in dictionary")
        } else {
            println!("Here is the 3 closest matches: ");
            for key in results {
                println!("{key}");
                println!("{}", map.get(key).unwrap());
            }
        }
    }
    println!("Goodbye!")
}
