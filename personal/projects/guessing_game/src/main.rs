use std::io; //io stands for input/output
// std is the standard library; learn more here: https://doc.rust-lang.org/std/prelude/index.html
// If it is not in the prelude bring it in using the `use` statement.


fn main() { // `fn` declares a new function, `main` is the main function
    println!("Guess the number!");
    println!("Please input your guess: ");
    let mut guess = String::new(); // variable to store the guess
    // variables are immutable by default, meaning once we give the variable a value, the value won't change
    // To make a variable mutable, we add mut before the variable name
    // :: syntx in the ::new indicates that new is an assocaited function of the String type
    // An associated function is a function that’s implemented on a type
    // new function create a new empty string
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
    // println!("You guessed: {}", guess);
}
