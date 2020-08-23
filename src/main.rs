use rand;
use rand::Rng;

fn main() {
    let mut rng_generator = rand::thread_rng();
    let secret_number = rng_generator.gen_range(0, 10);
    println!("Hello, world! = {}", secret_number);
}
