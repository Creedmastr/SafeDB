use std::fs;

pub fn init() {
    fs::write("./data.txt", "").expect("Error");
}
