use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut iteration_counter = 0;
    let secret_number = rand::rng().random_range(1..=100);

    loop {
	    println!("Please input your guess.");

	    let mut guess = String::new();

	    io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	    let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
	    };


            iteration_counter += 1;
	    println!("Guess #{iteration_counter} is {guess}");
	    match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => { 
			println!("\tYes, the Secret Number is {secret_number}");
			println!("\tYou win in {iteration_counter} tries!");
			break;
		} 

      }
   }

    
}
