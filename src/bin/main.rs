extern crate pythonkc;
extern crate time;

use std::thread;

use pythonkc::primes::{no_of_primes, no_of_primes_multi};

const BOUND: usize = 2_000_000;

fn main() {

    let t0 = time::now();
    let n = no_of_primes(BOUND);
    let a = (time::now() - t0).num_milliseconds() as f64 / 1_000.0;

    let t0 = time::now();
    let _ = no_of_primes_multi(BOUND);
    let b = (time::now() - t0).num_milliseconds() as f64 / 1_000.0;

    println!("\nRust alone:");
    println!("Single process took {:.3} seconds.", a);
    println!("Multiple processes took {:.3} seconds.\n", b);

    let percentage = n as f64 / BOUND as f64 * 100 as f64;

    thread::sleep(::std::time::Duration::from_secs(2));
    println!("And in case you were curious...");
    thread::sleep(::std::time::Duration::from_secs(2));
    println!("There are {} primes numbers below 2 million ({:.1}%)\n",
             n,
             percentage);
    thread::sleep(::std::time::Duration::from_secs(180));

}
