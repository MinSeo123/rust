use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Please input your guess.");
    //mut = 가변 변수를 만드는 방법.
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {}", guess);
}
