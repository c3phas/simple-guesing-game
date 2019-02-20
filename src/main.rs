//import necessary modules
use std::io;//for input/output
use rand::Rng;//used for the random number generator
use std::cmp::Ordering;


fn main() {
    println!("Guess the number");
	let secret_number = rand::thread_rng().gen_range(1,101);
//	println!("secret number is {}",secret_number);
//loop until user gets the correct number
	loop{
		println!("please input your guess");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
			//remove the whitespace and the newline character  and bind the new value to guess
		let guess: u32 = match guess.trim().parse(){
				Ok(num) => num,
				Err(_) => continue,
			};
		//	.expect("please type a number");
		//compare the value form the user with the secret number
		match guess.cmp(&secret_number){
			Ordering::Less => println!("too small"),
			Ordering::Greater => println!("too big"),
			Ordering::Equal => {
				println!("you win");
				break;
			}
		}
	}
}
