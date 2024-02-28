use std::fs;
use std::io;

fn main() -> io::Result<()>{

    // let file_name = "example.txt";
    // fs::write(file_name,"Hello World!");
    // println!("File {} is created succesfull",file_name);
    
    // fs::remove_file(file_name);
    // println!("file {} is deleted succesfully",file_name);

    let dir_name = "example_dir";
    fs::create_dir(dir_name);
    println!("Directory {} is created succesfully",dir_name);

    fs::remove_dir(dir_name);
    println!("Directory {} deleted succefully",dir_name);
 




    Ok(())

}
