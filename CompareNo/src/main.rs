use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){
    println!("Enter the number ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
}