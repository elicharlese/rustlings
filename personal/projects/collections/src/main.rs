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
// After these two lines, s will contain foobar.
// The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter.
// For example, in the code in Listing 8-16, we want to be able to use s2 after appending its contents to s1.

// let mut s1 = String::from("foo");
// let s2 = "bar";
// s1.push_str(s2);
// println!("s2 is {}", s2);

// If the push_str method took ownership of s2, we wouldn’t be able to print its value on the last line.
// However, this code works as we’d expect!

// The push method takes a single character as a parameter and adds it to the String.
// Listing 8-17 adds the letter “l” to a String using the push method.

// let mut s = String::from("lo");
// s.push('l');

// As a result, s will contain lol.

// Concatenation with the + Operator or the format! Macro
// Often, you’ll want to combine two existing strings. One way to do so is to use the + operator, as shown in Listing 8-18.

// let s1 = String::from("Hello, ");
// let s2 = String::from("world!");
// let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

// The string s3 will contain Hello, world!.
// The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator.
// The + operator uses the add method, whose signature looks something like this:

// fn add(self, s: &str) -> String {

// In the standard library, you'll see add defined using generics and associated types.
// Here, we’ve substituted in concrete types, which is what happens when we call this method with String values.

// When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
// If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:

// let s1 = String::from("tic");
// let s2 = String::from("tac");
// let s3 = String::from("toe");

// let s = s1 + "-" + &s2 + "-" + &s3;

// At this point, s will be tic-tac-toe.
// With all of the + and " characters, it’s difficult to see what’s going on. For more complicated string combining, we can instead use the format! macro:

// let s1 = String::from("tic");
// let s2 = String::from("tac");
// let s3 = String::from("toe");

// let s = format!("{}-{}-{}", s1, s2, s3);

// This code also sets s to tic-tac-toe.
// The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
// The version of the code using format! is much easier to read, and the code generated by the format! macro uses references so that this call doesn’t take ownership of any of its parameters.

// What is the difference between using a + b and a.push_str(b) to concatenate two strings?
// + consumes ownership of a, while push_str does not
// push_str takes &mut self while + takes self, so + consumes ownership and push_str does not.

// What is the maximum number of times a heap allocation could occur in this program? Write your answer in digits, e.g. 0 or 1.

// let s1 = String::from("tic");
// let s2 = String::from("tac");
// let s3 = String::from("toe");
// let s = s1 + "-" + &s2 + "-" + &s3;

// 7; One allocation for each call to String::from, and one allocation for every time + is called.

// Indexing into Strings
// In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation.
// However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.
// Consider the invalid code in Listing 8-19.

// let s1 = String::from("hello");
// let h = s1[0];

// The error and the note tell the story: Rust strings don’t support indexing.
// But why not? To answer that question, we need to discuss how Rust stores strings in memory.

// Internal Representation
// A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8 example strings from Listing 8-14. First, this one:

// let hello = String::from("Hola");

// In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long.
// Each of these letters takes 1 byte when encoded in UTF-8. The following line, however, may surprise you.
// (Note that this string begins with the capital Cyrillic letter Ze, not the Arabic number 3.)

// let hello = String::from("Здравствуйте");

// Asked how long the string is, you might say 12.
// In fact, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage.
// Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.
// To demonstrate, consider this invalid Rust code:

// let hello = "Здравствуйте";
// let answer = &hello[0];

// The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately,
// Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

// Bytes and Scalar Values and Grapheme Clusters! Oh My!

// Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes,
// scalar values, and grapheme clusters (the closest thing to what we would call letters).

// If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
// 224, 165, 135]

// That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:

// ['न', 'म', 'स', '्', 'त', 'े']

// There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own.
// Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

// ["न", "म", "स्", "ते"]

// Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.

// A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)).
// But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

// Slicing Strings
// Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.
// If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.

// Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

// let hello = "Здравствуйте";

// let s = &hello[0..4];

// Here, s will be a &str that contains the first 4 bytes of the string.
// Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд.

// If we were to try to slice only part of a character’s bytes with something like &hello[0..1], Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

// You should use ranges to create string slices with caution, because doing so can crash your program.

// Methods for Iterating Over Strings
// The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
// For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:

// for c in "Зд".chars() {
//     println!("{}", c);
// }

// This code will print the following:
// З
// д

// Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain:

// for b in "Зд".bytes() {
//     println!("{}", b);
// }

// This code will print the four bytes that make up this string:
// 208
// 151
// 208
// 180

// But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

// Getting grapheme clusters from strings as with the Devanagari script is complex, so this functionality is not provided by the standard library.
// Crates are available on crates.io if this is the functionality you need.

// Strings Are Not So Simple
// To summarize, strings are complicated.
// Different programming languages make different choices about how to present this complexity to the programmer.
// Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront.
// This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

// The good news is that the standard library offers a lot of functionality built off the String and &str types to help handle these complex situations correctly.
// Be sure to check out the documentation for useful methods like contains for searching in a string and replace for substituting parts of a string with another string.

// Let’s switch to something a bit less complex: hash maps!

// Which statement is the best explanation for why Rust does not allow string indexing?
// Indexing strings is ambiguous because strings represent several granularities of sequenced data
// A UTF-8 string can be interpreted as a sequence of bytes, characters, or grapheme clusters.
// None of these is necessarily the "default" way of interpreting a string, so a default indexing operation does not make sense.

// Which statement best describes the difference between the types of a string slice &str and a byte slice &[u8]?
// &str points to bytes that can always be interpreted as UTF-8, whereas &[u8] can be any byte sequence
// &str is a promise that the byte sequence it points to will always be valid UTF-8.
// Therefore a programmer who wants to e.g. print out an &str never needs to check if it is valid, or worry about accidentally interpreting an invalid string.

// Storing Keys with Associated Values in Hash Maps
// The last of our common collections is the hash map.
// The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.
// Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.

// Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.
// For example, in a game, you could keep track of each team’s score in a hash map in which each key is a team’s name and the values are each team’s score. Given a team name, you can retrieve its score.

// We’ll go over the basic API of hash maps in this section, but many more goodies are hiding in the functions defined on HashMap<K, V> by the standard library.
// As always, check the standard library documentation for more information.

// Creating a New Hash Map
// One way to create an empty hash map is using new and adding elements with insert.
// In Listing 8-20, we’re keeping track of the scores of two teams whose names are Blue and Yellow.

// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// Note that we need to first use the HashMap from the collections portion of the standard library.
// Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude.
// Hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.

// Just like vectors, hash maps store their data on the heap.
// This HashMap has keys of type String and values of type i32.
// Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

// Accessing Values in a Hash Map
// We can get a value out of the hash map by providing its key to the get method, as shown in Listing 8-21.

// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// let team_name = String::from("Blue");
// let score = scores.get(&team_name).copied().unwrap_or(0);

// We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:

// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// for (key, value) in &scores {
//     println!("{}: {}", key, value);
// }

// This code will print each pair in an arbitrary order:
// Yellow: 50
// Blue: 10

// Hash Maps and Ownership
// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.

// use std::collections::HashMap;

// let field_name = String::from("Favorite color");
// let field_value = String::from("Blue");

// let mut map = HashMap::new();
// map.insert(field_name, field_value);
// // field_name and field_value are invalid at this point, try using them and
// // see what compiler error you get!

// We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.

// If we insert references to values into the hash map, the values won’t be moved into the hash map.
// The values that the references point to must be valid for at least as long as the hash map is valid

// Updating a Hash Map
// Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time
// (but not vice versa: for example, both the Blue team and the Yellow team could have value 10 stored in the scores hash map).

// When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned.
// You could replace the old value with the new value, completely disregarding the old value.
// You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value.
// Or you could combine the old value and the new value.

// Overwriting a Value
// If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.
// Even though the code in Listing 8-23 calls insert twice, the hash map will only contain one key/value pair because we’re inserting the value for the Blue team’s key both times.

// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Blue"), 25);

// println!("{:?}", scores);

// This code will print {"Blue": 25}. The original value of 10 has been overwritten.

// Adding a Key and Value Only If a Key Isn’t Present
// It’s common to check whether a particular key already exists in the hash map with a value then take the following actions: if the key does exist in the hash map, the existing value should remain the way it is.
// If the key doesn’t exist, insert it and a value for it.

// Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
// The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
// Let’s say we want to check whether the key for the Yellow team has a value associated with it.
// If it doesn’t, we want to insert the value 50, and the same for the Blue team. Using the entry API, the code looks like Listing 8-24.

// use std::collections::HashMap;

// let mut scores = HashMap::new();
// scores.insert(String::from("Blue"), 10);

// scores.entry(String::from("Yellow")).or_insert(50);
// scores.entry(String::from("Blue")).or_insert(50);

// println!("{:?}", scores);

// The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
// This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

// Updating a Value Based on the Old Value
// Another common use case for hash maps is to look up a key’s value and then update it based on the old value.
// For instance, Listing 8-25 shows code that counts how many times each word appears in some text.
// We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word.
// If it’s the first time we’ve seen a word, we’ll first insert the value 0.

// use std::collections::HashMap;

// let text = "hello world wonderful world";

// let mut map = HashMap::new();

// for word in text.split_whitespace() {
//     let count = map.entry(word).or_insert(0);
//     *count += 1;
// }

// println!("{:?}", map);

// The split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the value in text.
// The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
// Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*).
// The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

// By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1.
// This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. 
// A hasher is a type that implements the BuildHasher trait.
// You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

// use std::collections::HashMap;
// fn main() {
//   let mut h = HashMap::new();
//   h.insert("k1", 0);
//   let v1 = &h["k1"];
//   h.insert("k2", 1);
//   let v2 = &h["k2"];
//   println!("{} {}", v1, v2);
// }

// This program does not compile.

// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

// use std::collections::HashMap;
// fn main() {
//     let mut h: HashMap<char, Vec<usize>> = HashMap::new();
//     for (i, c) in "hello!".chars().enumerate() {
//         h.entry(c).or_insert(Vec::new()).push(i);
//     }
//     let mut sum = 0;
//     for i in h.get(&'l').unwrap() {
//         sum += *i;
//     }
//     println!("{}", sum);
// }

// The output of this program will be: 5
// This program stores a vector of indexes for each occurrence of a given letter into a hashmap.
// Then it sums all the indexes for the letter 'l', which occurs at indexes 2 and 3 in the string "hello!".

// Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.
// Here are some exercises you should now be equipped to solve:

// - Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
// - Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
// - Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.

// The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!
