extern crate num_cpus;
extern crate rusty_primes;
extern crate time;

// use rusty_primes::{primes, primes_magic, primes_multi};

fn main() {
    //     let _ = num_cpus::get_physical() + 1;
    //     let bound = ::std::env::args()
    //         .nth(1)
    //         .unwrap_or_else(|| String::from("3000000"))
    //         .trim()
    //         .parse::<u32>()
    //         .expect("Must enter a valid bound");
    //     println!("\nRust alone:");
    //     let t0 = time::now();
    //     let _ = primes(bound);
    //     let t1 = (time::now() - t0).num_milliseconds() as f64 / 1_000.0;
    //     println!("Single process took {:.3} seconds.", t1);
    //     let t0 = time::now();
    //     let _ = primes_magic(bound);
    //     let t2 = (time::now() - t0).num_milliseconds() as f64 / 1_000.0;
    //     println!("'Magic' multiprocessing took {:.3} seconds.", t2);
    //     let t0 = time::now();
    //     let n = primes_multi(bound, 10);
    //     let t3 = (time::now() - t0).num_milliseconds() as f64 / 1_000.0;
    //     println!("'Manual' multiprocessing took {:.3} seconds.\n", t3);
    //     let percentage = n as f64 / bound as f64 * 100.0;
    //     println!("And in case you were curious...");
    //     println!("There are {} primes numbers below {} ({:.1}%)\n",
    //              n,
    //              bound,
    //              percentage);
}
