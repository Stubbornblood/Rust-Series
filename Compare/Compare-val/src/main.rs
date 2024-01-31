use std::io; //io stands for input output it is a module in standard liberary
use rand::Rng; // this one is use to generate random no as u can see it is not in standard lib
//so we will have to add using cargo we will use command 

fn main(){ // this one is main function which will run firstly when user execute the program
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //in above we are setting the range from 1 to 100 random no generate out of it
    println!("The secret number is : {}",secret_number);

    println!("Please input your guess!");
    let mut guess = String::new(); //initializing a new variable string variable named as guess

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    //in above code &mut this is reffrence we are reffering to guess and why we use mut in reffrece
    //because we want to make reffrence mutable too
    //read_line this one it to read line and expect this one to throw exception error if any occurs

    println!("You guessed:{}",guess);

}