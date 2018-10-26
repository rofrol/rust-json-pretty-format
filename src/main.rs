extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::fs;

fn main() {
    let json_file_path = Path::new("input.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let value: serde_json::Value = serde_json::from_reader(json_file).expect("error while reading json");
    let data = serde_json::to_string_pretty(&value).expect("Unable to pretty format json");
    fs::write("output.json", data).expect("Unable to write file");
}
