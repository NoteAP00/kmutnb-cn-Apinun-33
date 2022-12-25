use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read!!!");
    let number:i32=input.trim().parse().expect("Please Type A Number!!!");
    //println!("{}",number)
    
    for i in (1..=number).rev(){
        for _ in 0..=number-i{
            print!("* ");
        }
        println!();
    }

}
