fn divide(a:i32 , b:i32) -> Result<i32,&'static str> { //now lets do something here bonus
    if b == 0{
        Err("We am batman, and we love shellbng") 
    }
    else{
        Ok(a/b)
    }
}

// divide(a:i32 , b:i32) <- this is the function with name divide created by me 
//Have two argument i32 meaning is integer of 32 bits 
//Result<i32,&'static str> <- This represnt the retun type meaning 
//there can be error or success right so if the value is success what will be it's type
//i32 <- it will be integer why because if we divide integer with integer we will get integer
//if we get error it will retun a string &'static str 
//&'static str -> now break this code & represent reffrence reffrence to what to string slice
//static represnt that the code remain same till the program exist   if b == 0{
        // Err("Cannot divide by zero!")
    // }

//Now Lets make main funciton

fn main(){ // this is the fist function which will run when we do cargo runs

let result = divide(10,0).unwrap_or_else(|err|{
    println!("Error : {}",err);
    0
});
println!("Result : {}",result); // this is how we will print result 
}

//divide(10,2) we are calling the function
//unwrap_or_else we are unwraping the return in case of any error or_else this will execute and 
//(|err|{
    // println!("Error : {}",err);
    // 0
// }); we will get this value printed on terminal which is error and 0

//10/2 -> Result : 5
// evertime u do cargo run it will build and run the codeuse std::cmp::Ordering;