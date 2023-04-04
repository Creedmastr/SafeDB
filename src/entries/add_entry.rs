use std::{fs};

use super::entry::Entry;

pub fn add_entry(entry: String) {
    let mut file = fs::read_to_string("./data.txt").unwrap();
    
    let final_entry = Entry {
        id: 0,
        content: entry
    };

    file = file + &final_entry.to_string();

    fs::write("./data.txt", file).expect("");
}