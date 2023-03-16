// fn main() {
//     // let x = 5;
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6; // wont run because the x is immutable
//     println!("The value of x is: {x}");
// }

// Mutable and Immutable
// Adding mut also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.

// Constants
// You are not allowed to use mut with constants; they're always immutable
// Rust’s naming convention for constants is to use all uppercase with underscores between words.
// Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code. 
// Constants are valid for the entire time a program runs, within the scope they were declared in.
// const can be used in the global scope, and let can only be used in a function

// Shadowing
// We can shadow a variable by using the same variable’s name and repeating the use of the let keyword

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");

//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("{spaces}")
    // We can reuse names

    // let mut spaces = "   ";
    // let spaces = spaces.len();
    // println!("{}", spaces);
    // Compiles with an error `#[warn(unused_mut)]` on by default`
// }

// By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
// The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can 
// change the type of the value but reuse the same name.

// A variable cannot be assigned to a value of a different type than its original type.

// Data Types

// Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.
// when many types are possible, such as when we converted a String to a numeric type using parse, we must add a type annotation, like this:
// let guess: u32 = "42".parse().expect("Not a number!");
// If we don’t add the : u32 type annotation above, Rust will display the following error, which means the compiler needs more information
// from us to know which type we want to use:

// Scalar Type
// A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

// Integer Types
// An integer is a number without a fractional component.
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize

// Number literals and Example
// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

// Rust’s defaults are generally good places to start: integer types default to i32.
// The primary situation in which you’d use isize or usize is when indexing some sort of collection.

// Integer Overflow
// When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. 
// Instead, if overflow occurs, Rust performs two’s complement wrapping.
// - Wrap in all modes with the wrapping_* methods, such as wrapping_add
// - Return the None value if there is overflow with the checked_* methods
// - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
// - Saturate at the value’s minimum or maximum values with saturating_* methods

// Floating-Point Types
// Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
// The default type is f64.

// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

// Numeric Operations

// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let floored = 2 / 3; // Results in 0

//     // remainder
//     let remainder = 43 % 5;
// }

// The Boolean Type

// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

// The Character Type
// Rust’s char type is the language’s most primitive alphabetic type.

// fn main() {
//     let _c = 'z';
//     let _z: char = 'ℤ'; // with explicit type annotation
//     let _heart_eyed_cat = '😻';
//     println!("{_heart_eyed_cat}")
// }

// Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
// The largest number representable by the type i128 is:
// 2^127 - 1

// Compound Types
// Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

// The Tuple Types
// We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (_x, y, _z) = tup; // If variable is unused prefix it with an underscore

//     println!("The value of y is: {y}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;
//     println!("{five_hundred}");

//     let six_point_four = x.1;
//     println!("{six_point_four}");

//     let one = x.2;
//     println!("{one}");
// }

// The tuple without any values has a special name, unit.
// Expressions implicitly return the unit value if they don’t return any other value.

// The Array Type
// Unlike a tuple, every element of an array must have the same type.
// Unlike arrays in some other languages, arrays in Rust have a fixed length.

// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements.
// An array isn’t as flexible as the vector type, though.

// const months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

// // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

// const a: [i32; 5] = [1, 2, 3, 4, 5];

// // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
// const b: [i8; 2] = [3; 5];

//The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

// Accessing Array Elements
// An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. 
// You can access elements of an array using indexing, like this:

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }

// Invalid Array Element Access

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// This code compiles successfully.
// If you instead enter a number past the end of the array, such as 10, you’ll see output like this:
// thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// The program resulted in a runtime error at the point of using an invalid value in the indexing operation.
// If the index is greater than or equal to the length, Rust will panic.



// This is an example of Rust’s memory safety principles in action.
// In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed.

fn main() {
    let t = ([1; 2], [3; 4]);
    let (a, _) = t;
    println!("{}", a[0] + t.1[0]);
}

// The syntax [x; y] declares an array with y copies of the value x. The syntax (a, _) destructures t and binds a to [1; 2]. The syntax t.1 refers to the 1-th element of t, which is [3; 4].






