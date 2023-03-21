// Using Structs to Structure Related Data
// A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
// If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.
// Structs and enums (discussed in Chapter 6) are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile time type checking.

// Defining and Instantiating Structs
// Like tuples, the pieces of a struct can be different types.
// Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
// Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

// A struct’s name should describe the significance of the pieces of data being grouped together.
// Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
// To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.

// We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs, where the keys are the names of the fields and the values are the data we want to store in those fields.
// We don’t have to specify the fields in the same order in which we declared them in the struct.
// In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
// }

// To get a specific value from a struct, we use dot notation.
// If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");
// }

// Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
// As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// The active field gets the value of true, and the sign_in_count gets a value of 1.
// It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the email and username field names and variables is a bit tedious.
//  If the struct had more fields, repeating each name would get even more annoying.

// Using the Field Init Shorthand
// Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have the repetition of email and username

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// Creating Instances From Other Instances With Struct Update Syntax
// It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using struct update syntax.

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//     // --snip--

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
// }

// The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

// fn main() {
//     // --snip--

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }

// Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the "What Is Ownership?" section.

// Using Tuple Structs without Named Fields to Create Different Types
// Rust also supports structs that look similar to tuples, called tuple structs.
// Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
// Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

// To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple.

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// Note that the black and origin values are different types, because they’re instances of different tuple structs.
// Each struct you define is its own type, even though the fields within the struct might have the same types.
// Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.

// Unit-Like Structs Without Any Fields
// You can also define structs that don’t have any fields!
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// No need for curly brackets or parentheses! Then we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses.
// Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes.
// We wouldn’t need any data to implement that behavior!

// Ownership of Struct Data
// This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
// It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes

// Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
// Let’s say you try to store a reference in a struct without specifying lifetimes, like the following; this won’t work:

// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         email: "someone@example.com",
//         username: "someusername123",
//         active: true,
//         sign_in_count: 1,
//     };
// }

// Borrowing Fields of a Struct
// Similar to our discussion in "Different Tuple Fields", Rust's borrow checker will track ownership permissions at the both the struct-level and field-level.

// fn main() {
//     struct Point { x: i32, y: i32 }

//     let mut p = Point { x: 0, y: 0 };
//     let x = &mut p.x;
//     *x += 1;
//     println!("{}, {}", p.x, p.y);
// }

// As a result, if we try and use p while p.x is mutably borrowed like this:

// struct Point { x: i32, y: i32 }

// fn print_point(p: &Point) {
//     println!("{}, {}", p.x, p.y);
// }

// fn main() {
//     let mut p = Point { x: 0, y: 0 };
//     let x = &mut p.x;
//     print_point(&p);
//     *x += 1;
// }

// More generally, if you encounter an ownership error that involves a struct, you should consider which fields of your structure are supposed to be borrowed with which permissions.
// But be aware of the borrow checker's limitations, since Rust may sometimes assume more fields are borrowed than they actually are.

// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let mut p = Point { x: 1, y: 2 };
//     let x = &mut p.x;
//     let y = &mut p.y;
//     *x += 1;
//     *y += 1;
//     println!("{} {}", p.x, p.y);
// }

// Rust understands that .x refers to a different object than .y, so it is valid to take simultaneous mutable references to both fields.


































