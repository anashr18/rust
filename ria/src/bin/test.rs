// use std::

use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) <= time_limit {
        count += 1;
    }
    println!("{}", count);

    let a = 42;
    let r = &a;
    let b = a + *r;

    println!("a + a = {}", b);
    println!("{}", r);
    println!("{}", a);

    let needle = 877;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
