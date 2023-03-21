// Fixing Ownership Errors
// Learning how to fix an ownership error is a core Rust skill. When the borrow checker rejects your code, how should you respond?
// Rust will always reject an unsafe program1.
// But sometimes, Rust will also reject a safe program.

// Fixing an Unsafe Program: Returning a Reference to the Stack

// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

// Our first case study is about returning a reference to the stack.
// When thinking about how to fix this function, we need to ask: why is this program unsafe?
// If you want to pass around a reference to a string, you have to make sure that the underlying string lives long enough.

// Depending on the situation, here are four ways you can extend the lifetime of the string. One is to move ownership of the string out of the function, changing &String to String

// fn return_a_string() -> String {
//     let s = String::from("Hello world");
//     s
// }

// Another possibility is to return a string literal, which lives forever (indicated by 'static).
// This solution applies if we never intend to change the string, and the string can be written directly in the program source code:

// fn return_a_string() -> &'static str {
//     "Hello world"
// }

// Another possibility is to defer lifetime-checking to runtime by using garbage collection.

// use std::rc::Rc;

// fn return_a_string() -> Rc<String> {
//     let s = Rc::new(String::from("Hello world"));
//     Rc::clone(&s)
// }

// In short, Rc::clone only clones a pointer to s and not the data itself.
// At runtime, the Rc checks when the last Rc pointing to data has been dropped, and then deallocates the data.

// Yet another possibility is to have the caller provide a "slot" to put the string using a mutable reference:

// fn return_a_string(output: &mut String) {
//     output.replace_range(.., "Hello world");
// }

// With this strategy, the caller is responsible for creating space for the string.
// This style can be verbose, but it can also be more memory-efficient if the caller needs to carefully control when allocations occur.

// But the key idea is to recognize the root issue underlying the surface-level ownership error.
// But the key idea is to recognize the root issue underlying the surface-level ownership error.

// Fixing an Unsafe Program: Not Enough Permissions
// Another common issue is trying to mutate read-only data, or trying to drop data behind a reference.
// This function is supposed to create a person's full name from a vector of name parts, including an extra title.

// fn stringify_name_with_title(name: &mut Vec<String>) -> String {
// // fn stringify_name_with_title(name: &Vec<String>) -> String { // possible solution
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
// This program is rejected by the borrow checker because name is an immutable reference, but name.push(..) requires the W permission.
// This program is unsafe because push could invalidate other references to name outside of stringify_name_with_title, like this:

// fn main() {
//     let name = vec![String::from("Ferris")];
//     let first = &name[0]; // L1
//     stringify_name_with_title(&name); // L2
//     println!("{}", first); // L3
// }

// One straightforward solution is to change the type of name from &Vec<String> to &mut Vec<String>:
// But this is also not a good solution!

// It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String.
// This version of stringify_name_with_title would make the input name unusable, which is very annoying to a caller as we discussed at the beginning of "References and Borrowing".

// So the choice of &Vec is actually a good one, which we do not want to change. Instead, we can change the body of the function.
// There are many possible fixes which vary in how much memory they use. One possibility is to clone the input name:

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut name_clone = name.clone();
//     name_clone.push(String::from("Esq."));
//     let full = name_clone.join(" ");
//     full
// }

// By cloning name, we are allowed to mutate the local copy of the vector.
// However, the clone copies every string in the input. We can avoid unnecessary copies by adding the suffix later:

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut full = name.join(" ");
//     full.push_str(" Esq.");
//     full
// }

// In general, writing Rust functions is a careful balance of asking for the right level of permissions.

// Which of the following is NOT a valid kind of fix to the issue of returning a stack reference from a function?
// Extend the lifetime of the stack frame
// A stack frame cannot have its lifetime extended, so that is not a valid solution.

// Let's say a programmer tried writing the following function:

/// Rounds all the floats in a vector to the nearest integer, in-place
// fn round_in_place(v: &Vec<f32>) {
//     for n in v {
//         *n = n.round();
//     }
// }

// The Rust compiler rejects their code with the following error:
// Given the stated purpose of the function, which of the following would be the most idiomatic fix to the program?

// fn round_in_place(v: &mut Vec<f32>) {
//     for n in v {
//         *n = n.round();
//     }
// }

// Fixing an Unsafe Program: Aliasing and Mutating a Data Structure

// Another unsafe operation is using a reference to heap data that gets deallocated by another alias.

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// Note: this example uses iterators and closures to succinctly find a reference to the largest string
// Again, we should ask: why is this program unsafe? Because dst.push(..) could deallocate the contents of dst, invalidating the reference largest.
// To fix the program, the key insight is that we need to shorten the lifetime of largest to not overlap with dst.push(..). One possibility is to clone largest:

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// However, this may cause a performance hit for allocating and copying the string data.
// Another possibility is to perform all the length comparisons first, and then mutate dst afterwards:

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//     let to_add: Vec<String> =
//         src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
//     dst.extend(to_add);
// }

// However, this may cause a performance hit for allocating and copying the string data.
// Another possibility is to perform all the length comparisons first, and then mutate dst afterwards:

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//     let to_add: Vec<String> = 
//         src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
//     dst.extend(to_add);
// }

// However, this also causes a performance hit for allocating the vector to_add.
// A final possibility is to copy out the length, since we don't need the contents of largest, just its length. This solution is arguably the most idiomatic and the most performant.

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
//     for s in src {
//         if s.len() > largest_len {
//             dst.push(s.clone());
//         }
//     }
// }

// These solutions all share in common the key idea: shortening the lifetime of borrows on dst to not overlap with a mutation to dst.

// Fixing an Unsafe Program: Copying vs. Moving Out of a Collection
// A common confusion for Rust learners happens when copying data out of a collection, like a vector.

// fn main() {
//     let v: Vec<i32> = vec![0, 1, 2];
//     let n_ref: &i32 = &v[0];
//     let n: i32 = *n_ref;
// }

// But what happens if we change the type of elements in the vector from i32 to String? Then it turns out we no longer have the necessary permissions:

// fn main() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let s_ref: &String = &v[0];
//     let s: String = *s_ref;
// }

// The first program will compile, but the second program will not compile. Rust gives the following error message:
// But references are non-owning pointers — we can't take ownership through a reference.

// Therefore Rust complains that we "cannot move out of [...] a shared reference".
// But why is this unsafe? We can illustrate the problem by simulating the rejected program:

// fn main() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let s_ref: &String = &v[0];
//     let s: String = *s_ref; // L1

//     // These drops are normally implicit, but we've added them for clarity.
//     drop(s); // L2
//     drop(v); // L3
// }

// What happens here is a double-free.
// After executing let s = *s_ref, both v and s think they own "Hello world". After s is dropped, "Hello world" is deallocated.
// Then v is dropped, and undefined behavior happens when the string is freed a second time.

// In technical terms, Rust says that the type i32 implements the Copy trait, while String does not implement Copy (we will discuss traits in a later chapter).

// In sum, if a value does not own heap data, then it can be copied without a move. For example:
// - An i32 does not own heap data, so it can be copied without a move.
// - A String does own heap data, so it can not be copied without a move.
// - An &String does not own heap data, so it can be copied without a move.

// Note: One exception to this rule is mutable references. For example, &mut i32 is not a copyable type. So if you do something like:

// fn main() {
//     let mut n = 0;
//     let a = &mut n;
//     let b = a;
// }

// Then a cannot be used after being assigned to b. That prevents two mutable references to the same data from being used at the same time.

// So if we have a vector of non-Copy types like String, then how do we safely get access to an element of the vector? 
// Here's a few different ways to safely do so. First, you can avoid taking ownership of the string and just use an immutable reference:

// fn main() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let s_ref: &String = &v[0];
//     println!("{s_ref}!");
// }

// Second, you can clone the data if you want to get ownership of the string while leaving the vector alone:

// fn main() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let mut s: String = v[0].clone();
//     s.push('!');
//     println!("{s}");
// }

// Finally, you can use a method like Vec::remove to move the string out of the vector:

// fn main() {
//     let mut v: Vec<String> = vec![String::from("Hello world")];
//     let mut s: String = v.remove(0);
//     s.push('!');
//     println!("{s}");
//     assert!(v.len() == 0);
// }

// Fixing a Safe Program: Mutating Different Tuple Fields
// Rust may also reject safe programs. One common issue is that Rust tries to track permissions at a fine-grained level.
// However, Rust may conflate two different paths as the same path.

// Let's first look at an example of fine-grained permission tracking that passes the borrow checker.
// This program shows how you can borrow one field of a tuple, and write to a different field of the same tuple:

// fn main() {
//     let mut name = (
//         String::from("Ferris"),
//         String::from("Rustacean")
//     );
//     let first = &name.0;
//     name.1.push_str(", Esq.");
//     println!("{first} {}", name.1);
// }

// However, Rust can lose track of exactly which paths are borrowed.
// fn get_first(name: &(String, String)) -> &String {
// &name.0
// }

// fn main() {
//     let mut name = (
//         String::from("Ferris"),
//         String::from("Rustacean")
//     );
//     let first = get_first(&name);
//     name.1.push_str(", Esq.");
//     println!("{first} {}", name.1);
// }

// Now we can't do name.1.push_str(..)! Rust will return this error:
// That's strange, since the program was safe before we edited it. The edit we made doesn't meaningfully change the runtime behavior.
// The problem is that Rust doesn't look at the implementation of get_first when deciding what get_first(&name) should borrow.
// Rust only looks at the type signature, which just says "some String in the input gets borrowed".

// Remember, the key idea is that the program above is safe. It has no undefined behavior!
// A future version of Rust may be smart enough to let it compile, but for today, it gets rejected.
// So how should we work around the borrow checker today?

// Another possibility is to defer borrow checking to runtime with cells, which we will discuss in future chapters.

// Fixing a Safe Program: Mutating Different Array Elements
// A similar kind of problem arises when we borrow elements of an array.

// fn main() {
//     let mut a = [0, 1, 2, 3];
//     let x = &mut a[0];
//     *x += 1;
//     println!("{a:?}");
// }

// It uses a single path a[_] that represents all indexes of a. Rust does this because it cannot always determine the value of an index.

// fn main() {
//     let idx = a_complex_function();
//     let x = &mut a[idx];
// }

// What is the value of idx? Rust isn't going to guess, so it assumes idx could be anything.

// fn main() {
//     let mut a = [0, 1, 2, 3];
//     let x = &mut a[0];
//     let y = &a[1];
//     *x += *y;
// }

// However, Rust will reject this program because a gave its read permission to x. The compiler's error message says the same thing:
// Again, this program is safe. For cases like these, Rust often provides a function in the standard library that can work around the borrow checker.

// fn main() {
//     let mut a = [0, 1, 2, 3];
//     let (x, rest) = a.split_first_mut().unwrap();
//     let y = &rest[0];
//     *x += *y;
// }

// You might wonder, but how is split_first_mut implemented?
// In some Rust libraries, especially core types like Vec or slice, you will often find unsafe blocks.
// `unsafe` blocks allow the use of "raw" pointers, which are not checked for safety by the borrow checker.

// fn main() {
//     let mut a = [0, 1, 2, 3];
//     let x = &mut a[0] as *mut i32;
//     let y = &a[1] as *const i32;
//     unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
// }

// Unsafe code is sometimes necessary to work around the limitations of borrow checker.
// As a general strategy, let's say the borrow checker rejects a program you think is actually safe.
// Then you should look for standard library functions (like split_first_mut) that contain unsafe blocks which solve your problem.
// For now, just be aware that unsafe code is how Rust implements certain otherwise-impossible patterns.

// Which of the following best explains why an i32 can be copied without a move, but a String cannot?
// A String owns data on the heap, while an i32 does not

// When fixing an ownership error, you should ask yourself: is my program actually unsafe?
// If yes, then you need to understand the root cause of the unsafety. If no, then you need to understand the limitations of the borrow checker to work around them.
