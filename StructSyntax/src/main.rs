struct System {
    active : bool,
    name : String,
    email : String,
    sign_in_count : u32,
}

fn main(){
    let user1 = System{
        active : true,
        name : String::from("Elvise Boi"),
        email : String::from("ElviseBoi@gmai.com"),
        sign_in_count : 1,
    };
   
//    println!("Name of admin user is : {}",user1.name);

    let System {active,name,..} = user1;

    println!("Name is : {}",name);
}