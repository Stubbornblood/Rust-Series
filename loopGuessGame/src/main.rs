use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess secret number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Enter your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Not able to get number");
        let guess:u32 = guess.trim().parse().expect("not able to convert");
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Guess number is Less then secret number : {secret_number}"),
            Ordering::Equal =>{ println!("Guess number is equal to secret number : {secret_number} you win 🗿👍");
            break;},
            Ordering::Greater => println!("Guess number is Greater then secret number : {secret_number}")
        }
       
    }

}