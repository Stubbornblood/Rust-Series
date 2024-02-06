use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed : {guess}");
    let num = guess.trim().parse::<i32>().unwrap();
    match num.cmp(&secret_number){
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Equal"),
    }
}