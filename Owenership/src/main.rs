fn main() {
    // Demonstrating ownership in Rust
    // Integer type
    let x: i32 = 5; // x is a binding with an integer value
    let y: i32 = x; // y is a copy of the value of x
    println!("Value of x is: {x} and value of y is: {y}");

    // String type
    let s1: String = String::from("hello"); // s1 is a binding with a String value
    let s2: String = s1.clone(); // s2 is a clone of the value of s1, creating a deep copy
    println!("{s2}"); // Print the cloned value of s1
    println!("{s1}"); // Print the original value of s1, which remains valid
}
