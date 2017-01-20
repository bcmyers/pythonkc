extern crate cheddar;

use std::fs;
use std::fs::{copy, File};
use std::io::Write;

fn main() {
    let header: String = cheddar::Cheddar::new()
        .expect("could not read manifest")
        .module("ffi")
        .expect("malformed module path")
        .compile_code()
        .expect("could not compile code");
    fs::create_dir_all("../data").unwrap();
    let mut file = File::create("../data/pythonkc.h").expect("unable to create file");
    file.write_all(header.as_bytes()).expect("unable to write data");
}
