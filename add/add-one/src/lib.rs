extern crate rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

use rand::Rng;

pub fn add_rand(x: i32) -> i32 {
    let mut rng = rand::thread_rng();
    x + rng.gen_range(1,10)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4,add_one(3));
    }

    use super::*;

    #[test]
    fn add_rand_test() {
        assert!(add_rand(3) <= 13);
        assert!(3 < add_rand(3));
    }
}