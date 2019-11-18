use std::io; // capture user's input and provides outupt to cli
use rand::Rng; // random number dependency held by cargo , cargo is a package manager just like npm

fn main(){
    println!("Guess the number");

    let  secret_number = rand::thread_rng().gen_range(1,100);

    println!("Secret number is  :{}", secret_number);

    println!("Please input your guess");

    let mut guess = String::new(); //def for mutable variable
    io::stdin().read_line(&mut guess)
        .expect("failed to read the line"); //error handling

    println!("You  Guessed:{}", guess); // {} is a place holder


}
