use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
struct Definition {
    def: Vec<String>,
}

pub fn convert() {
    let mut dict: HashMap<String, Definition> = HashMap::new();
    let file = File::open("src/data.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let _bytes_read = reader
        .read_to_string(&mut buf)
        .expect("Error reading to buf");
    dbg!(buf.len());
    let mut last_key = String::new();
    let mut last_value: Vec<String> = Vec::new();

    while buf.len() > 4911089 {
        let delim = buf.find(", \"").expect("find failed to locate item");
        dbg!(&delim);
        let entry = &buf[..delim];

        let entry: Vec<_> = entry.split(":").collect();
        dbg!(&entry);
        dbg!(&entry.len());

        if entry.len() >= 2 {
            dict.insert(
                last_key.clone(),
                Definition {
                    def: vec![entry[1].to_owned()],
                },
            );
            last_key = entry[0].to_owned();
            last_value = vec![entry[1].to_owned()];
            println!("add 0 to key and 1 to value in hashmap")
        } else if entry.len() == 1 {
            last_value.push(entry[0].to_owned());
            dict.insert(
                last_key.clone(),
                Definition {
                    def: last_value.clone(),
                },
            );
            println!("add 0 to last_key value")
        } else if entry.len() == 0 {
            println!("continue");
            continue;
        }

        buf = buf[delim + 1..].to_owned();
    }
    // dbg!(dict);
}
