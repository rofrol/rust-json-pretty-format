extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::fs;
use std::env;
use std::error::Error;
use std::process;

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
    let output_option = file_stem.to_str()
        .map(|x:&str| x.to_owned() + ".pretty.")
        .and_then(|x:String| extension.to_str().and_then(|y:&str| Some(x + y)))
        .map(|x:String| canon_parent.join(x));
    if let Some(output) = output_option {
        fs::write(&output, data).expect("Unable to write file");
    } else {
        eprintln!("There was problem with generating output name. Exiting.");
        process::exit(1);
    }
    Ok(())
}
