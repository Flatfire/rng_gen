use rand::{thread_rng, Rng};
use std::{self, io};

const RNGTYPE: [&str; 3] = ["Signed Int", "Unsigned Int", "Float"];
const RNGSIZE: [&str; 4] = ["8", "16", "32", "64"];

// Establish default returns for polymorphic generate_num
// Each call to thread_rng().gen() derives the return type from the implementation
trait GenRand<T> {
    fn generate_num() -> T;
}
struct RandNum {}

// Unsigned Ints
impl GenRand<u8> for RandNum {
    fn generate_num() -> u8 {
        return thread_rng().gen();
    }
}

impl GenRand<u16> for RandNum {
    fn generate_num() -> u16 {
        return thread_rng().gen();
    }
}

impl GenRand<u32> for RandNum {
    fn generate_num() -> u32 {
        return thread_rng().gen();
    }
}

impl GenRand<u64> for RandNum {
    fn generate_num() -> u64 {
        return thread_rng().gen();
    }
}

// Signed Ints
impl GenRand<i8> for RandNum {
    fn generate_num() -> i8 {
        return thread_rng().gen();
    }
}

impl GenRand<i16> for RandNum {
    fn generate_num() -> i16 {
        return thread_rng().gen();
    }
}

impl GenRand<i32> for RandNum {
    fn generate_num() -> i32 {
        return thread_rng().gen();
    }
}

impl GenRand<i64> for RandNum {
    fn generate_num() -> i64 {
        return thread_rng().gen();
    }
}

// Floats

impl GenRand<f32> for RandNum {
    fn generate_num() -> f32 {
        return thread_rng().gen();
    }
}

impl GenRand<f64> for RandNum {
    fn generate_num() -> f64 {
        return thread_rng().gen();
    }
}

fn main() {
    while true {
    let mut x = String::new(); // Type selector
    let mut y = String::new(); // Bit length selector

    println!("This program generates random numbers.");

        println!("What kind of number do you want?");
        for i in 1..=RNGTYPE.len() {
            println!("{}. {}", i, RNGTYPE[i - 1]); // Print type options
        }
        io::stdin().read_line(&mut x);
        let x = x.trim();
        println!();
        println!("How long should it be?");
        if x != "3" {
            for i in 1..=RNGSIZE.len() {
                println!("{}. {}", i, RNGSIZE[i - 1]); //print size options
            }
        } else {
            println!("{}. {}", 1, RNGSIZE[2]); //print size options
            println!("{}. {}", 2, RNGSIZE[3]); //print size options
        }
        io::stdin().read_line(&mut y);
        let y = y.trim();
        println!();

        // Parse selection
        if x == "1" {
            if y == "1" {
                let rnum: u8 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else if y == "2" {
                let rnum: u16 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else if y == "3" {
                let rnum: u32 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else if y == "4" {
                let rnum: u64 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            }
        } else if x == "2" {
            if y == "1" {
                let rnum: i8 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else if y == "2" {
                let rnum: i16 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else if y == "3" {
                let rnum: i32 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else if y == "4" {
                let rnum: i64 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else {
                println!("Invalid option.")
            }
        } else if x == "3" {
            if y == "1" {
                let rnum: f32 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else if y == "2" {
                let rnum: f64 = RandNum::generate_num();
                println!("Your number is: {}", rnum);
            } else {
                println!("Invalid option.")
            }
        } else {
            println!("Invalid option.")
        }
    }
}
