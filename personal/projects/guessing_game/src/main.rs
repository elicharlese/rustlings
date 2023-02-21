use std::io; //io stands for input/output
// std is the standard library; learn more here: https://doc.rust-lang.org/std/prelude/index.html
// If it is not in the prelude bring it in using the `use` statement.


fn main() { // `fn` declares a new function, `main` is the main function
    println!("Guess the number!");
    println!("Please input your guess: ");
    let mut guess = String::new(); // variable to store the guess
    // variables are immutable by default, meaning once we give the variable a value, the value won't change
    // To make a variable mutable, we add mut before the variable name
    // :: syntax in the ::new indicates that new is an associated function of the String type
    // An associated function is a function that’s implemented on a type
    // new function create a new empty string
    io::stdin() // we could use std::io::stdin() to read from the standard input
    // std::io:Stdin is a type that implements the Read trait
        .read_line(&mut guess)
        // Hence, you need to write &mut guess rather than &guess to make it mutable.
        .expect("Failed to read line");
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {guess}");
    // println!("You guessed: {}", guess);
}

// An associated function is a function that’s implemented on a type, in this case String.
// This new function creates a new, empty string.
// You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.
// References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.

// Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. 
// We call each possible state a variant.
// The purpose of these Result types is to encode error-handling information.





