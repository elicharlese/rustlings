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

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// Loops Labels to Disambiguate Between Multiple Loops
// If you have loops within loops, break and continue apply to the innermost loop at that point.
// You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
// Loop labels must begin with a single quote.

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// Conditional Loops with while
// However, this pattern is so common that Rust has a built-in language construct for it, called a while loop.

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// While a condition holds true, the code runs; otherwise, it exits the loop.

// Looping Through a Collection with for
// You can choose to use the while construct to loop over the elements of a collection, such as an array.

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// As a more concise alternative, you can use a for loop and execute some code for each item in a collection.

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// The way to do that would be to use a Range, provided by the standard library, which generates all numbers in sequence 
// starting from one number and ending before another number.

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// rev, to reverse the range
