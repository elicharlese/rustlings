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

fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}

// With this strategy, the caller is responsible for creating space for the string.
// This style can be verbose, but it can also be more memory-efficient if the caller needs to carefully control when allocations occur.

// But the key idea is to recognize the root issue underlying the surface-level ownership error.
// But the key idea is to recognize the root issue underlying the surface-level ownership error.

// Fixing an Unsafe Program: Not Enough Permissions
// Another common issue is trying to mutate read-only data, or trying to drop data behind a reference.
// This function is supposed to create a person's full name from a vector of name parts, including an extra title.

fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."







































































































