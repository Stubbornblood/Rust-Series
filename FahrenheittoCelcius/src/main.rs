use std::io;

fn main(){
    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("not able to take value");
    let f:f64 = f.trim().parse().expect("not able to convert to float");
    let c = (5.0/9.0)*(f-32.0);
    println!("{c}");
}