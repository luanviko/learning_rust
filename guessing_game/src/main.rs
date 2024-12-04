use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guessing Game");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Clean and convert
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)  => continue,
        }; 
    
        match guess.cmp(&secret_number){
            Ordering::Less    => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too big!"), 
            Ordering::Equal   => {
                println!("Correct! You win!");
                break;
            }
        }
    }
    
    

}
