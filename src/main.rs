extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::fs;
use std::env;
use std::error::Error;
use std::process;
use std::path::PathBuf;

fn main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let json_file = File::open(input)?;
    let value: serde_json::Value = serde_json::from_reader(json_file)?;
    let data = serde_json::to_string_pretty(&value)?;
    let output_option = insert_segment_before_extension(".pretty.", &input);
    if let Ok(output) = insert_segment_before_extension(".pretty.", &input) {
        fs::write(&output, data)?;
    } else {
        eprintln!("There was problem with generating output name");
        process::exit(1);
    }
    Ok(())
}

fn insert_segment_before_extension(segment: &str, input: &Path) -> Result<PathBuf, Box<Error>> {
    let canon = fs::canonicalize(&input)?;
    let canon_parent = Path::parent(&canon).expect("Cannot get parent");
    let file_stem  = input.file_stem().expect("Cannot get file_stem");
    let extension = input.extension().expect("Cannot get extension");
    let output_option = file_stem.to_str()
        .map(|x:&str| x.to_owned() + segment)
        .and_then(|x:String| extension.to_str().and_then(|y:&str| Some(x + y)))
        .map(|x:String| canon_parent.join(x));
    if let Some(output) = output_option {
        Ok(output)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "There was problem with generating output name")))
    }
}
