// Common Collections
// Rust’s standard library includes a number of very useful data structures called collections.
// Most other data types represent one specific value, but collections can contain multiple values.
// Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
// Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.

// In this chapter, we’ll discuss three collections that are used very often in Rust programs:
// - A vector allows you to store a variable number of values next to each other.
// - A string is a collection of characters.
// - We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
// - A hash map allows you to associate a value with a particular key.
// - It’s a particular implementation of the more general data structure called a map.

// Storing Lists of Values with Vectors
// The first collection type we’ll look at is Vec<T>, also known as a vector.
// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
// Vectors can only store values of the same type.

// Creating a New Vector
// To create a new empty vector, we call the Vec::new function, as shown in Listing 8-1.

// let v: Vec<i32> = Vec::new();

// Note that we added a type annotation here.
// Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.
// This is an important point. Vectors are implemented using generics; we’ll cover how to use generics with your own types in Chapter 10.
// For now, know that the Vec<T> type provided by the standard library can hold any type. When we create a vector to hold a specific type, we can specify the type within angle brackets.
// In Listing 8-1, we’ve told Rust that the Vec<T> in v will hold elements of the i32 type.

// More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation.
// Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.

// let v = vec![1, 2, 3];

// Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary. Next, we’ll look at how to modify a vector.

// Updating a Vector
// To create a vector and then add elements to it, we can use the push method

// let mut v = Vec::new();

// v.push(5);
// v.push(6);
// v.push(7);
// v.push(8);

// As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword, as discussed in Chapter 3.
// The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.

// Reading Elements of Vectors
// There are two ways to reference a value stored in a vector: via indexing or using the get method.
// In the following examples, we’ve annotated the types of the values that are returned from these functions for extra clarity.

// both methods of accessing a value in a vector, with indexing syntax and the get method.

// let v = vec![1, 2, 3, 4, 5];

// let third: &i32 = &v[2];
// println!("The third element is {}", third);

// let third: Option<&i32> = v.get(2);
// match third {
//     Some(third) => println!("The third element is {}", third),
//     None => println!("There is no third element."),
// }

// Note a few details here. We use the index value of 2 to get the third element because vectors are indexed by number, starting at zero.
// Using & and [] gives us a reference to the element at the index value.
// When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.

// The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements.

// let v = vec![1, 2, 3, 4, 5];

// let does_not_exist = &v[100];
// let does_not_exist = v.get(100);

// When we run this code, the first [] method will cause the program to panic because it references a nonexistent element.
// This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.

// When the get method is passed an index that is outside the vector, it returns None without panicking.
// You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
// Your code will then have logic to handle having either Some(&element) or None, as discussed in Chapter 6.
// For example, the index could be coming from a person entering a number.
// If they accidentally enter a number that’s too large and the program gets a None value, you could tell the user how many items are in the current vector and give them another chance to enter a valid value.
// That would be more user-friendly than crashing the program due to a typo!

// When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid.
// Recall the rule that states you can’t have mutable and immutable references in the same scope.
// That rule applies in Listing 8-6, where we hold an immutable reference to the first element in a vector and try to add an element to the end.
// This program won’t work if we also try to refer to that element later in the function:

// let mut v = vec![1, 2, 3, 4, 5];

// let first = &v[0];

// v.push(6);

// println!("The first element is: {}", first);

// This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements
// to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored.
// In that case, the reference to the first element would be pointing to deallocated memory.
// The borrowing rules prevent programs from ending up in that situation.

// Iterating over the Values in a Vector
// To access each element in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time.
// Listing 8-7 shows how to use a for loop to get immutable references to each element in a vector of i32 values and print them.

// let v = vec![100, 32, 57];
// for n_ref in &v {
//     // n_ref has type &i32
//     let n_plus_one: i32 = *n_ref + 1;
//     println!("{}", n_plus_one);
// }

// To read the number that n_ref refers to, we have to use the * dereference operator to get to the value in n_ref before we can add 1 to it
// We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
// The for loop in Listing 8-8 will add 50 to each element.

// let mut v = vec![100, 32, 57];
// for n_ref in &mut v {
//     // n_ref has type &mut i32
//     *n_ref += 50;
// }

// To change the value that the mutable reference refers to, we again use the * dereference operator to get to the value in n_ref before we can use the += operator.

// Safely Using Iterators
// For now, one important detail is that iterators contain a pointer to data within the vector.
// We can see how iterators work by desugaring a for-loop into the corresponding method calls of Vec::iter and Iterator::next:

// fn main() {
//     use std::slice::Iter;
//     let mut v: Vec<i32>         = vec![1, 2];
//     let mut iter: Iter<'_, i32> = v.iter();
//     let n1: &i32                = iter.next().unwrap();
//     let n2: &i32                = iter.next().unwrap();
//     let end: Option<&i32>       = iter.next();
// }

// Observe that the iterator iter is a pointer that moves through each element of the vector.
// The next method advances the iterator and returns an optional reference to the previous element, either Some (which we unwrap) or None at the end of the vector.
// This detail is relevant to safely using vectors

// fn dup_in_place(v: &mut Vec<i32>) {
//     for n_ref in v.iter() {
//         v.push(*n_ref);
//     }
// }

// So to use iterators safely, Rust does not allow you to add or remove elements from the vector during iteration.
// One way to iterate over a vector without using a pointer is with a range, like we used for string slices in Chapter 4.4.

// fn main() {
//     use std::ops::Range;
//     let mut v: Vec<i32>        = vec![1, 2];
//     let mut iter: Range<usize> = 0 .. v.len();
//     let i1: usize              = iter.next().unwrap();
//     let n1: &i32               = &v[i1];
// }

// Using an Enum to Store Multiple Types
// Vectors can only store values that are the same type.
// This can be inconvenient; there are definitely use cases for needing to store a list of items of different types.
// Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!

// We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum.
// Then we can create a vector to hold that enum and so, ultimately, holds different types.
// We’ve demonstrated this in Listing 8-9.

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// const row = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("blue")),
//     SpreadsheetCell::Float(10.12),
// ];

// println!(SpreadsheetCell);

// Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element.
// We must also be explicit about what types are allowed in this vector.
// If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector.
// Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled.

// If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work.

// Now that we’ve discussed some of the most common ways to use vectors, be sure to review the API documentation for all the many useful methods defined on Vec<T> by the standard library.
// For example, in addition to push, a pop method removes and returns the last element.

// Dropping a Vector Drops Its Elements
// Like any other struct, a vector is freed when it goes out of scope, as annotated in Listing 8-10.

// {
//     let v = vec![1, 2, 3, 4];

//     // do stuff with v
// } // <- v goes out of scope and is freed here

// When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.
// The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

// Which call to this find_until function will cause a runtime panic?

// fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
//   for i in 0 .. til {
//     if v[i] == n {
//       return Some(i);
//     }
//   }
//   return None;
// }

// find_until(&vec![1, 2, 3], 4, 4)

// If til = 4, then for a vector of length 3, the for-loop will attempt to index the vector with i = 3, which is out of bounds.
//  This function does not panic if n = 1 because it returns before reaching the out-of-bounds index.

// Storing UTF-8 Encoded Text with Strings
// New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8.

// We discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.
// In this section, we’ll talk about the operations on String that every collection type has, such as creating, updating, and reading.
// We’ll also discuss the ways in which String is different from the other collections, namely how indexing into a String is complicated by the differences between how people and computers interpret String data.

// What Is a String?
// Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.
// String literals, for example, are stored in the program’s binary and are therefore string slices.

// The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
// When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.

// Creating a New String
// Many of the same operations available with Vec<T> are available with String as well, because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.

// let mut s = String::new();

// This line creates a new empty string called s, which we can then load data into.
// Often, we’ll have some initial data that we want to start the string with.
// For that, we use the to_string method, which is available on any type that implements the Display trait, as string literals do.
// Listing 8-12 shows two examples.

// let data = "initial contents";

// let s = data.to_string();

// // the method also works on a literal directly:
// let s = "initial contents".to_string();

// This code creates a string containing initial contents.

// We can also use the function String::from to create a String from a string literal.
// The code in Listing 8-13 is equivalent to the code from Listing 8-12 that uses to_string.

// let s = String::from("initial contents");

// Because strings are used for so many things, we can use many different generic APIs for strings, providing us with a lot of options.
// Some of them can seem redundant, but they all have their place!
// In this case, String::from and to_string do the same thing, so which you choose is a matter of style and readability.

// Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as shown in Listing 8-14.

// let hello = String::from("السلام عليكم");
// let hello = String::from("Dobrý den");
// let hello = String::from("Hello");
// let hello = String::from("שָׁלוֹם");
// let hello = String::from("नमस्ते");
// let hello = String::from("こんにちは");
// let hello = String::from("안녕하세요");
// let hello = String::from("你好");
// let hello = String::from("Olá");
// let hello = String::from("Здравствуйте");
// let hello = String::from("Hola");

// All of these are valid String values.

// Updating a String
// A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it.
// In addition, you can conveniently use the + operator or the format! macro to concatenate String values.

// Appending to a String with push_str and push
// We can grow a String by using the push_str method to append a string slice, as shown in Listing 8-15.

// let mut s = String::from("foo");
// s.push_str("bar");












































