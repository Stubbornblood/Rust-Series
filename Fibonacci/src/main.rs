fn fibonacci(n:i32)->i32{

    if n==0{
        return 0;
    }else if n==1{
        return 1;
    }else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

fn main(){
    let n = 10;
    println!("the : {}",fibonacci(n));
}