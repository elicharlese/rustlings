// The Slice Type
// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// A slice is a kind of reference, so it is a non-owning pointer.

// write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
// Without slices, we might write the signature of the function like this:

// fn first_word(s: &String) -> ?
// The first_word function has a &String as a parameter.
// We don’t want ownership of the string, so this is fine. But what should we return?
// We don’t really have a way to talk about part of a string. 
// However, we could return the index of the end of the word, indicated by a space. Let’s try that

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// For now, know that iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead.
// The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
// This is a bit more convenient than calculating the index ourselves.

// Because the enumerate method returns a tuple, we can use patterns to destructure that tuple.
// In the for loop, we specify a pattern that has i for the index in the tuple and &item for the single byte in the tuple.
// Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
// }

// This program compiles without any errors, as s retains write permissions after calling first_word.
// Because word isn’t connected to the state of s at all, word still contains the value 5.
// We could use that value 5 with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in word.

// Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!
// Managing these indices is even more brittle if we write a second_word function.
// Its signature would have to look like this: fn second_word(s: &String) -> (usize, usize) {

// Now we’re tracking a starting and an ending index, and we have even more values that were calculated from data in a particular state but aren’t tied to that state at all.
// We have three unrelated variables floating around that need to be kept in sync.

// Luckily, Rust has a solution to this problem: string slices.

// String Slices
// A string slice is a reference to part of a String, and it looks like this:

// fn main() {
//     let s = String::from("hello world");
//     let hello: &str = &s[0..5];
//     let world: &str = &s[6..11];
//     let s2: &String = &s;
// }

// We create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
// Slices are special kinds of references because they are "fat" pointers, or pointers with metadata.
// Here, the metadata is the length of the slice. We can see this metadata by changing our visualization to peek into the internals of Rust's data structures:

// Observe that the variables hello and world have both a ptr and a len field, which together define the underlined regions of the string on the heap.
// You can also see here what a String actually looks like: a string is a vector of bytes (Vec<u8>), which contains a length len and a buffer buf that has a pointer ptr and a capacity cap.

// Because slices are references, they also change the permissions on referenced data.

// fn main() {
//     let mut s = String::from("hello");
//     let hello: &str = &s[0..5];
//     println!("{hello}");
//     s.push_str(" world");
// }

// Range Syntax
// With Rust’s .. range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:

// let s = String::from("hello");

// let slice = &s[0..2];
// let slice = &s[..2];

// By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:

// let s = String::from("hello");

// let len = s.len();

// let slice = &s[3..len];
// let slice = &s[3..];

// You can also drop both values to take a slice of the entire string. So these are equal:

// let s = String::from("hello");

// let len = s.len();

// let slice = &s[0..len];
// let slice = &s[..];

// Note: String slice range indices must occur at valid UTF-8 character boundaries.

// Rewriting first_word with string slices
// With all this information in mind, let’s rewrite first_word to return a slice.
// The type that signifies “string slice” is written as &str:

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.
// Now when we call first_word, we get back a single value that is tied to the underlying data.
// The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

// Returning a slice would also work for a second_word function:

// fn second_word(s: &String) -> &str {

// We now have a straightforward API that’s much harder to mess up, because the compiler will ensure the references into the String remain valid.

// Slices make this bug impossible and let us know we have a problem with our code much sooner. For example:

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
//     println!("the first word is: {}", word);
// }

// You can see that calling first_word now removes the write permission from s, which prevents us from calling s.clear(). Here’s the compiler error:

// Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference.
// Because clear needs to truncate the String, it needs to get a mutable reference.
// The println! after the call to clear uses the reference in word, so the immutable reference must still be active at that point.
// Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.
// Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

// String Literals Are Slices
// Recall that we talked about string literals being stored inside the binary.
// Now that we know about slices, we can properly understand string literals:

// let s = "Hello, world!";

// The type of s here is &str: it’s a slice pointing to that specific point of the binary.
// This is also why string literals are immutable; &str is an immutable reference.

// String Slices as Parameters
// Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:
// fn first_word(s: &String) -> &str {

// A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.
// fn first_word(s: &str) -> &str {

// Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

// Other Slices
// String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:
// let a = [1, 2, 3, 4, 5];

// Just as we might want to refer to a part of a string, we might want to refer to part of an array. We’d do so like this:
// let a = [1, 2, 3, 4, 5];
// let slice = &a[1..3];
// assert_eq!(slice, &[2, 3]);

// This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length.

// Consider the variables s2 and s3 in the following program.
// These two variables will be located in memory within the stack frame for main.
// Each variable has a size in memory on the stack, not including the size of pointed data.
// Which statement is true about the sizes of s2 and s3?

// fn main() {
//     let s = String::from("hello");
//     let s2: &String = &s;
//     let s3: &str = &s[..];
// }

// s3 has more bytes than s2
// fn main() {
//   println!(
//     "&String={} &str={}",
//     std::mem::size_of::<&String>(),
//     std::mem::size_of::<&str>(),
//   );
// }You can verify this yourself using std::mem::size_of, like so:

// Also, note that Rust will implicitly convert string references to either &String or &str based on the context of the reference.
// So the expression &s produces two different values based on the expected type of &s.

// The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
// The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data
// automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.































