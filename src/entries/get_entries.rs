use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

use super::entry::Entry;

fn parse_entries(data_file: &str) -> Vec<Entry> {
    let mut result_content: Vec<String> = vec![];
    let mut result_id: Vec<String> = vec![];

    let mut file = std::fs::read_to_string(data_file).unwrap();
    file = file.replace("{", "").replace("}", "").replace(r#"""#, "");

    std::fs::write("./temp", file).expect("");

    let temp_f = fs::File::open("./temp").unwrap();

    let buf_reader = BufReader::new(File::from(temp_f));

    for line in buf_reader.lines() {
        match line.as_ref().unwrap() {
            x if x.contains("content: ") => {
                result_content.push(line.unwrap().replace("content: ", "").replace("    ", ""));
            }

            x if x.contains("id: ") => {
                result_id.push(line.unwrap().replace("id: ", "").replace("    ", ""));
            }

            _ => {}
        }
    }

    std::fs::remove_file("./temp").expect("");

    let mut result = vec![];
    let mut index = 0;

    for _item in result_id.clone() {
        let non_mut_result_id = result_id.clone();
        result.push(
            Entry {
                id: result_id[index].replace(r#"""#, "").replace(", ", "").parse().unwrap(),
                content: result_content[index].clone()
            }
        );

        index += 1;
    }

    result
}

pub fn get_entries() -> Vec<Entry> {
    return parse_entries("./data.txt");
}
