use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
	
	let number = rand::thread_rng()
				.gen_range(1..=100);
	
	println!("number is {number}");
	
	
	loop{
		println!("Input Guess: ");
		let mut guess = String::new(); //sets 'guess' to the result of calling 'String::new()'

		io::stdin()
			.read_line(&mut guess)
			.expect("ah broke"); // this is all technically part of one logical line.

		/*
		This:

		io::stdin()
			.read_line(&mut guess)
			.expect("ah broke");

			is the same as this:

			io::stdin().read_line(&mut guess).expect("ah broke"); 

			*/

		/*the variable 'guess' is 'shadowing' the original guess.  
		trim() removes white space.
		the parse() method on strings converts string to another type.  
		*/

		let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {guess}");

		match guess.cmp(&number) {
			Ordering::Less => println!("too low"),
			Ordering::Greater => println!("too high"),
			Ordering::Equal => {
				println!("Dead nuts on");
				break;
				}
			}
	}
	
}
