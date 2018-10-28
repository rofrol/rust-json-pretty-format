extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::fs;
use std::env;
use std::error::Error;


fn main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let json_file = File::open(input).expect("file not found");
    let value: serde_json::Value = serde_json::from_reader(json_file).expect("error while reading json");
    let data = serde_json::to_string_pretty(&value).expect("Unable to pretty format json");
    let canon = fs::canonicalize(&input).unwrap();
    let canon_parent = Path::parent(&canon).unwrap();
    let file_stem  = input.file_stem().unwrap();
    let extension = input.extension().unwrap();
    let output_name = file_stem.to_string_lossy() + ".pretty." + extension.to_string_lossy();
    let output = canon_parent.join(output_name.as_ref());
    fs::write(&output, data).expect("Unable to write file");
    Ok(())
}
