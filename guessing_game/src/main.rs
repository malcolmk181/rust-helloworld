use std::io;

fn main() {
    println!("Guess the number!"); // ! makes println a macro rather than a function?

    println!("Please input your guess.");

    let mut guess = String::new(); // by adding mut, this variable's value can change

    io::stdin()
        .read_line(&mut guess) // references are by default immutable! mut allows us to change guess
        .expect("Failed to read line"); // expect will be called if return value of above is Err
        // will get a compile warning without an expect / error handling!!!! so cool!

    println!("You guessed: {guess}");
}
