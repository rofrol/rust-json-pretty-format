extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::fs;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let json_file = File::open(input)?;
    let value: serde_json::Value = serde_json::from_reader(json_file)?;
    let data = serde_json::to_string_pretty(&value)?;
    insert_segment_before_extension(".pretty.", &input)
        .and_then(|output| Ok(fs::write(&output, data)?))
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
    match output_option {
        Some(output) => Ok(output),
        None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "There was a problem with generating output name"))),
    }
}
