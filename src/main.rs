use std::io;

fn main() {
    println!("Guess a number!");
    println!("Go ahead and type it in!");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your line!");

    println!("You guessed:{}", guess);
}
