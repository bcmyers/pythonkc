extern crate cheddar;

use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let header: String = cheddar::Cheddar::new()
        .expect("could not read manifest")
        .module("ffi")
        .expect("malformed module path")
        .compile_code()
        .expect("could not compile code");
    for directory in ["debug", "release"].iter() {
        fs::create_dir_all(format!("target/{}", directory)).unwrap();
        let mut file = File::create(format!("target/{}/libpythonkc.h", directory))
            .expect("unable to create file");
        file.write_all(header.as_bytes()).expect("unable to write data");
    }
}
