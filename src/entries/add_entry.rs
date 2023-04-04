use std::fs;

use super::{entry::{Entry, EntryConversions}, get_entries::get_entries};
use rand;

pub fn add_entry(entry: String) {
    let mut file = fs::read_to_string("./data.txt").unwrap();
    let mut final_entry: Entry;

    final_entry = entry.to_entry();

    let entries = get_entries();

    for entry in entries {
        if entry.id == final_entry.id {
            final_entry.id = rand::random::<u32>()
        }
    }

    file = file + &final_entry.to_string();

    fs::write("./data.txt", file).expect("");
}
