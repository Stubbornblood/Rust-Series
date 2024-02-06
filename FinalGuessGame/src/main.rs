use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("Guessing number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("{secret_number}");
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("not getting values");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{println!("Please Enter integer"); continue;},
    };

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Guess number is smaller then secret number {secret_number}"),
        Ordering::Equal => {
            println!("Guess number is equal to secret number {secret_number}");
            break;
        },
        Ordering::Greater => println!("Guess number is Greater then secret number {secret_number}"),
    }
}
}