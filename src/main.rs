use std::io;

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    loop{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("s");

        let x: i32 = match s.trim().parse::<i32>(){
            Ok(x) => x,
            Err(_) => {
            println!("Nan");
            return;
            }
        };
        
        if x == -1{break;}
        vec.push(x);
    }

    let mut sum: i32 = 0;
    for i in 0..vec.len(){
        sum += vec[i];
    }
    println!("{}", sum);
}
