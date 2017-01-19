extern crate pythonkc;
extern crate time;

use pythonkc::primes;

fn main() {
    let t0 = time::now();
    let x = primes::no_of_primes_multi(3_000_000, 10);
    let t1 = (time::now() - t0).num_milliseconds() as f64 / 1_000.0;
    println!("Result = {}\tTook {:.3} seconds", x, t1);
}
