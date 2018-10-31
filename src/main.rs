extern crate serde;
extern crate serde_json;

use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
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
    let file_stem = input.file_stem().expect("Cannot get file_stem");
    let extension = input.extension().expect("Cannot get extension");
    // expected trait std::error::Error, found struct `std::io::Error`
    // Ok(Command...?) converts between errors
    // https://stackoverflow.com/questions/48430836/rust-proper-error-handling-auto-convert-from-one-error-type-to-another-with-que/48431339#48431339
    // https://github.com/rust-lang-nursery/error-chain/issues/119#issuecomment-274957996
    Ok(file_stem
        .to_str()
        .ok_or(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed on file_stem.to_str()",
        ))).map(|x: &str| x.to_owned() + segment)
        .and_then(|x: String| {
            extension
                .to_str()
                .ok_or(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed on extension.to_str()",
                ))).map(|y: &str| x + y)
        }).map(|x: String| canon_parent.join(x))?)
}
