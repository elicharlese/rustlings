// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// Parameters

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// In function signatures, you must declare the type of each parameter.

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// Statements and Expressions
// Function bodies are made up of a series of statements optionally ending in an expression.
// Because Rust is an expression-based language, this is an important distinction to understand.

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resulting value.

// Statements do not return values.
// Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:

// fn main() {
//     let x = (let y = 6);
// }
// The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.
// This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. 
// In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

// Calling a function is an expression. Calling a macro is an expression.
// A new scope block created with curly brackets is an expression

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1 // Expressions do not include ending semicolons.
//     };

//     println!("The value of y is: {y}");
// }

// If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

// Functions with Return Values
// Functions can return values to the code that calls them.
// We don’t name return values, but we must declare their type after an arrow (->).
// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// Running this code will print The value of x is: 6. But if we place a
// semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error.

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }

// The main error message, “mismatched types,” reveals the core issue with this code.
// The definition of the function plus_one says that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the unit type.

// Comments
// For comments that extend beyond a single line, you’ll need to include // on each line, like this:






































