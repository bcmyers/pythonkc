extern crate cheddar;

use std::fs;
use std::io::Write;

const NAME: &'static str = "rusty_primes";

fn main() {
    let header: String = cheddar::Cheddar::new()
        .expect("could not read manifest")
        .module("ffi")
        .expect("malformed module path")
        .compile_code()
        .expect("could not compile code");
    let modes: Vec<&str> = vec!["debug", "release"];
    for mode in modes {
        fs::create_dir_all(format!("target/{}", mode)).unwrap();
        let mut file = fs::File::create(format!("target/{}/{}.h", mode, NAME))
            .expect("unable to create file");
        file.write_all(header.as_bytes()).expect("unable to write data");

    }
}
