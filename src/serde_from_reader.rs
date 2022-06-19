use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Dict {
    word: String,
    def: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Dict, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let d = serde_json::from_reader(reader)?;

    // Return the `Dict`.
    Ok(d)
}

pub fn from_reader() {
    // convert();
    let d = read_user_from_file("src/test.json.small").unwrap();
    println!("{:#?}", d);
}
