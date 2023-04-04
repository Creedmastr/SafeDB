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
