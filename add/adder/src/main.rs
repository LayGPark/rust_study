use add_one;
use rand::Rng;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("Hello, world! {} plus rand is {}!", num, add_one::add_rand(num));

    let mut rng = rand::thread_rng();
    println!("Hello, world! rand is {}!", rng.gen_range(0,10));
    println!("Hello, world!");

}
