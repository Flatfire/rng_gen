use rand::Rng;

fn main() {

    println!("This program generates random numbers.");
    println!("Today's random number is: {}", generate_unum());

    
}

fn generate_unum() -> u32 {
    
    let mut rnum = rand::thread_rng(); // Creates a random number generator
    let x = rnum.gen::<u32>(); // Assigns random number to 

    return x;
}