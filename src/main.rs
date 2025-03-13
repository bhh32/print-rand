use rand::Rng;

fn main() {
    // Generate a random number between 1 and 50
    let rand_num: i32 = rand::random();
    // Print the random number that was generated
    println!("Random number: {rand_num}");
}
