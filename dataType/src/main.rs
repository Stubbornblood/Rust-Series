// fn main(){
//     let sum = 4+10;
//     println!("{sum}");
//     let diffrent = 95.5-4.3;
//     println!("{diffrent}");
//     let product = 4 * 30;
//     println!("{product}");
//     let quotient = 56.7/32.2;
//     println!("{quotient}");
//     let truncated = -4/2;
//     println!("{truncated}");
//     let t = true;
//     let f: bool = false;
//     println!("{t},{f}");
//     let c = 'z';
//     let z:char = 'Z';
//     let heart_eyed_cat = '😻';
//     println!("{c},{z},{heart_eyed_cat}")

// }

fn main(){
    let tup: (i32,f64,u8) = (500,6.5,1);
    let {x,y,z} = tup;
    println!("{}",x);
}