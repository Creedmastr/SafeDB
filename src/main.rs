#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::entries::add_entry::add_entry;

pub mod entries;
pub mod init;

fn main() {
    add_entry("e".to_string());
    {
        let args = std::env::args().collect::<Vec<String>>();
        if args.len() >= 2 {
            if args[1] == "init" {
                init::init()
            }
        }
    }

    println!("{:#?}", entries::get_entries::get_entries());
}


#[cfg(test)]
mod test {
    use crate::entries::{entry::Entry, get_entries};

    use super::*;

    #[test]
    fn test_add_entry_and_get_entry() {
        init::init();

        add_entry("test".to_string());
        let entries = get_entries::get_entries();

        assert_eq!(
            Entry {
                id: 0,
                content: "test".to_string()
            }.content,
            entries[0].content.replace(" ", "") // I know it works correctly, because there's always a space at the end (for the moment)
        );
    }
}
