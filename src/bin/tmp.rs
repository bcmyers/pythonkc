extern crate pythonkc;
extern crate time;

use pythonkc::primes;

fn main() {
    for i in 3..20 {
        let t0 = time::now();
        let x = primes::no_of_primes_multi(5_000_000, i);
        let t1 = (time::now() - t0).num_milliseconds() as f64 / 1_000.0;
        println!("{} Result = {}\tTook {:.3} seconds", i, x, t1);
    }
}
