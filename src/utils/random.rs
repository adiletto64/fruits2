use rand::prelude::*;
use rand::distributions::{Bernoulli, Distribution};


pub fn randint(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}


pub fn probably(salt: f64) -> bool {
    let d = Bernoulli::new(salt).unwrap();
    return d.sample(&mut rand::thread_rng());
}
