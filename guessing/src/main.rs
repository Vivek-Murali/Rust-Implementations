use rand::Rng;
use std::io;


fn main() {
    println!("Guess the random number from 0-10");
    // creating a random number
    let secret = rand::thread_rng ().gen_range(0..=10);
    println!("The secret number has been generated");

    let mut user_number = String::new();

    println!("Guess the secret number!!");

    io::stdin().read_line(&mut user_number).expect("Failed to read the secret");


    if secret.to_string().eq(&user_number.trim())  {
        println!("Congratulations! You guessed the correct number");
    }else {
        println!("Sorry, You guess it wrong");
        println!("The secret number is {}", secret.to_string());
    }
}
