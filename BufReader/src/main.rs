use std::fs::File;
use std::io::{BufReader, Read};
use std::io::Error; // Add missing import for std::io::Error
fn main() -> Result<(), Error> { // Handle File::open error by returning an Err variant with the error message
    // Open the input file
    let input_file = File::open("./index.txt")?;

    // Create a BufReader for efficient reading
    let mut buf_reader = BufReader::new(input_file);

    // Define a buffer to hold data read from the input file
    let mut buffer = [0; 10]; // Let's assume a small buffer size for this example

    // Read data from the input file using BufReader
    let bytes_read = buf_reader.read(&mut buffer)?;

    println!("Bytes read: {}", bytes_read);
    println!("Content read: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    Ok(())
}
