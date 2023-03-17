// Control Flow

// if Expressions

// fn main() {
//     // let number = 3; // condition was true
//     let number = 7; // condition was false

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// All if expressions start with the keyword if, followed by a condition.
// Blocks of code associated with the conditions in if expressions are sometimes called arms,
// just like the arms in match expressions that we discussed
// Optionally, we can also include an else expression, which we chose to do here, to give the
// program an alternative block of code to execute should the condition evaluate to false.
// If you don’t provide an else expression and the condition is false, the program will just
// skip the if block and move on to the next bit of code.

// It’s also worth noting that the condition in this code must be a bool.
// Rust will not automatically try to convert non-Boolean types to a Boolean.
// You must be explicit and always provide if with a Boolean as its condition.

// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// Handling Multiple Conditions with else if

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// When this program executes, it checks each if expression in turn and executes the first body for which the condition holds true.
// That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.

// Using if in a let statement
// Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// This means the values that have the potential to be results from each arm of the if must be the same type

// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

// fn main() {
//     let x = 1;
//     let y = if x { 0 } else { 1 };
//     println!("{y}");
// }

// This program does not compile. The condition to an if-expression must be a boolean. Rust does not have a concept of "truthy" or "falsy" values.
