fn main() {
    let number = 42;

    match number {
        0 => print!("It's zero!"),
        _ => print!("it is something else")
                
    }
}
// fn main() {
//     let number = 42;

//     match number {
//         0 => println!("It's zero!"),
//         1 | 2 => println!("It's one or two!"),
//         3..=10 => println!("It's between 3 and 10 (inclusive)!"),
//         n if n % 2 == 0 => println!("It's an even number!"),
//         _ => println!("It's something else!"),
//     }
// }
