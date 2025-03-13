use rand::Rng;

fn main() {
    // Generate a random number between 1 and 50
    let rand_num = rand::rng().random_range(1..=50);
    // Print the random number that was generated
    println!("Random number: {rand_num}");
}
