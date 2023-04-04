use std::{io::{BufReader, BufRead}, fs::{File, self}};

fn parse_entries(data_file: &str) -> Vec<String> {
    let mut result = vec![];

    let mut file = std::fs::read_to_string(data_file).unwrap();
    file = file
        .replace("{", "")
        .replace("}", "")
        .replace(r#"""#, "");

    std::fs::write("./temp", file).expect("");

    let temp_f = fs::File::open("./temp").unwrap();

    let buf_reader = BufReader::new(File::from(temp_f));

    for line in buf_reader.lines() {
        if line.as_ref().unwrap().contains("content:") {
            result.push(line.unwrap().replace("content: ", "").replace("    ", ""));
        }
    } 

    std::fs::remove_file("./temp").expect("");

    result
}

pub fn get_entries() -> Vec<String> {
    return parse_entries("./data.txt");
}
