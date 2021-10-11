use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let mut guess= String::new();

    io::stdin().read_line(&mut guess).expect("Input Error");
    let secret = rand::thread_rng().gen_range(0..101);
    println!("Your output {} and secret is {}",guess,secret)
}
