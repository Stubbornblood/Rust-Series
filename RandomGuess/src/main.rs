use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main(){
    println!("Guessing value....");
    let secret_value = rand::thread_rng().gen_range(1..=100);
    // println!("{secret_value}");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    // let guess:u32 = guess.trim().parse().expect("Please type a number.");
    // println!("You guessed : {guess}");
    let guess = rand::thread_rng().gen_range(1..=100);
    match guess.cmp(&secret_value){
        Ordering::Less => println!("Guess value is equal to secret value is : {secret_value}"),
        Ordering::Equal => println!("Secret value is {secret_value} is equal to guess no!"),
        Ordering::Greater => println!("Guess no is greater then Secret value is : {secret_value}")
    }
}