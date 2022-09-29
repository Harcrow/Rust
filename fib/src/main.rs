use std::io;

fn main() {
    println!("Enter the Nth number of the fibonacci sequence to go. ");
	let mut guess = String::new(); //sets 'guess' to the result of calling 'String::new()'
	let mut first = 0;
	let mut second = 1;
	let mut fib = 0;
	io::stdin()
		.read_line(&mut guess)
		.expect("ah broke"); // this is all technically part of one logical line.

	let guess: u32 = guess.trim().parse().expect("Enter a number");
	println!("N is {guess} ");
	
	
	
	for _number in 1..guess{
		fib = first + second;
		first = second;
		second = fib;
	}
	println!("Fibonnaci of {guess} is {fib}");
		
}
