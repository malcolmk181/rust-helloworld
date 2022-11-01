use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); // ! makes println a macro rather than a function?

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // by adding mut, this variable's value can change

    io::stdin()
        .read_line(&mut guess) // references are by default immutable! mut allows us to change guess
        .expect("Failed to read line"); // expect will be called if return value of above is Err
        // will get a compile warning without an expect / error handling!!!! so cool!

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
