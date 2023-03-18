// Fixing Ownership Errors
// Learning how to fix an ownership error is a core Rust skill. When the borrow checker rejects your code, how should you respond?
// Rust will always reject an unsafe program1.
// But sometimes, Rust will also reject a safe program.

// Fixing an Unsafe Program: Returning a Reference to the Stack

fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}

// If you want to pass around a reference to a string, you have to make sure that the underlying string lives long enough.






























































































