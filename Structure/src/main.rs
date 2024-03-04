#[derive(Debug)]
struct User{
    active : bool,
    username : String,
    email : String,
    sign_in_count : u32,
}

fn buid_user(email: String, username : String) -> User{
    User{
        active : true,
        username : username,
        email : email,
        sign_in_count : 1,
    }
}

fn main(){
    let user1 = buid_user(String::from("drJhatka@gmail.com"),String::from("drJhatka"));
    println!("Data is : {:?}",user1);
}