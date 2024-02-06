use std::io;
fn main(){
    loop{    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
        Ok(num)=> num,
        Err(_) => {println!("Please guess again");continue;},
    };
    println!("You guessed : {guess}");}
}