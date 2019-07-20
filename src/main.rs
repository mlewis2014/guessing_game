use std::io;
use std::cmp::Ordering;
use rand::Rng;

// guessing game
fn main() {
    //println!("Hello, world!");
    println!("Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
	    println!("Enter your best guess now: ");

	 		let mut guess = String::new();

	 		io::stdin().read_line(&mut guess)
	    		.expect("Failed to read line");

	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num, 
	    	Err(_) => continue, 
	    
	    };

	    println!("You guessed: {}", guess);

	    match guess.cmp(&secret_number) {
	    		Ordering::Less => println!("Too small!"),
	        Ordering::Greater => println!("Too big!"),
	        Ordering::Equal => { 
	        	println!("You win!");
	        	break;
	        }
	    }
		}

}

