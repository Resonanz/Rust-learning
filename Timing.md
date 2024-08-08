```
// Source: https://docs.rs/timeit/0.1.2/timeit/

// The timeit macro will determine the number of loops automatically
use timeit::{timeit, timeit_loops};

pub fn run_timeit_macro() {
    println!("Running run_timeit_macro...");
    timeit!({
        let mut x: Vec<u64> = Vec::new();
        for i in 0..500_000 {
            // try 500_000_000 -- it takes 9 seconds and 4 GB memory on Ubuntu System Monitor
            //print!(".");
            //if i == 0 { print!("*") };
            x.push(i);
        }
    });
}

// The timeit_loops macro runs a specified number of loops and saves the elapsed time to a variable
pub fn run_timeit_loops_macro() {
    println!("Running run_timeit_loops_macro...");
    let sec = timeit_loops!(100, {
        let mut x: Vec<u64> = Vec::new();
        for i in 0..1000 {
            x.push(i);
        }
    });

    print!("Elapsed time = {}", sec);
}

```
