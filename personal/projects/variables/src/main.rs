fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // wont run because the x is immutable
    println!("The value of x is: {x}");
}
// You are not allowed to use mut with constants; they're always immutable
// Rust’s naming convention for constants is to use all uppercase with underscores between words.
// Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code. 
