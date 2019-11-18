use std::io;

fn main(){
    println!("Guess the number");
    println!("Please input your guess");

    let mut guess = String::new(); //def for mutable variable
    io::stdin().read_line(&mut guess).expect("failed to read the line");
    println!("You  Guessed:{}", guess);
}
