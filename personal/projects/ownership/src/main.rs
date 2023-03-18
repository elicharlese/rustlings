// What is Ownership?
// Ownership is a discipline for ensuring the safety of Rust programs.

// Safety is the Absence of Undefined Behavior
// This program is safe to execute:

// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

// fn main() {
//     let x = true;
//     read(x);
// }

// We can make this program unsafe to execute by moving the call to read before the definition of x:

// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

// fn main() {
//     read(x); // oh no! x isn't defined!
//     let x = true;
// }

// When programs are executed by an interpreter, reading x before it's defined would usually raise
// an exception such as Python's NameError or Javascript's ReferenceError.
// But these safeguards come at a cost.
// Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible.
// Therefore Rust does not check at runtime whether a variable is defined before being used. Instead, Rust checks at compile-time.

// Let's first consider how the safe program compiles and executes. On a computer with a processor using an x86 architecture, Rust generates
// the following assembly code for the main function in the safe program (see the full assembly code here):

// On a computer with a processor using an x86 architecture, Rust generates the following assembly code for the main function in the safe program
// (see the full assembly code here https://rust.godbolt.org/z/xnT1fzsqv):

// main:
//     ; ...
//     mov     edi, 1
//     call    read
//     ; ...

// This assembly code will:
// Move the number 1, representing true, into a "register" (a kind of assembly variable) called edi.
// Call the read function, which expects its first argument y to be in the edi register.

// If the unsafe function was allowed to compile, its assembly might look like this:

// main:
//     ; ...
//     call    read
//     mov     edi, 1    ; mov is after call
//     ; ...

// When read wants to use its argument y for any purpose, it will immediately cause UNDEFINED BEHAVIOR!

// Undefined behavior is especially dangerous for low-level programs with direct access to memory.
// Catching bugs at compile-time means avoiding those bugs in production, improving the reliability of your software.
// Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.

// Ownership as a Discipline for Memory Safety
// The Rust Reference maintains a large list of "Behavior considered undefined". https://doc.rust-lang.org/reference/behavior-considered-undefined.html

// Memory is the space where data is stored during the execution of a program.
// "memory is an array of bytes" or "memory is the pointers I get back from malloc".

// Variables Live in the Stack

// fn main() {
//     let n = 5; // L1
//     let y = plus_one(n); //L3
//     println!("The value of y is: {y}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1 // L2
// }

// Variables live in frames. A frame is a mapping from variables to values within a single scope, such as a function.
// Frames are organized into a stack of currently-called-functions.
// After a function returns, Rust deallocates the function's frame.
// Deallocation is also called freeing or dropping, and we use those terms interchangeably.
// This sequence of frames is called a stack because the most recent frame added is always the next frame freed.

// When an expression reads a variable, the variable's value is copied from its slot in the stack frame.

// fn main() {
//     let a = 5; // L1
//     let mut b = a; // L2
//     b += 1; //L3
// }

// Boxes Live in the Heap
// However, copying data can take up a lot of memory.

// fn main() {
//     let a = [0; 1_000_000];
//     let b = a;
// }

// Observe that copying a into b causes the main frame to contain 2 million elements.
// To transfer access to data without copying it, Rust uses pointers.

// A pointer is a value that describes a location in memory.
// The heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a specific stack frame.
// Rust provides a construct called Box for putting data on the heap.

// fn main() {
//     let a = Box::new([0; 1_000_000]);
//     let b = a;
// }


// At runtime, nothing happens to a when it is moved.
// At runtime, a move is just a copy. At compile-time, a move is a transfer of ownership.

// Boxes are used by Rust data structures1 like Vec, String, and HashMap to hold a variable number of elements.


// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first);
//     println!("{full}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// Variables Cannot Be Used After Being Moved
// The string program helps illustrate a key safety principle for ownership. Imagine that first were used in main after calling add_suffix.

// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first);
//     println!("{full}, originally {first}"); // first is now used here
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// undefined behavior: pointer used after its pointee is freed
// So if you move a variable, Rust will stop you from using that variable later.

// Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.
// Moving ownership of heap data avoids undefined behavior from reading deallocated memory.

// Cloning Avoids Moves
// One way to avoid moving data is to clone it using the .clone() method.

// fn main() {
//     let first = String::from("Ferris");
//     let first_clone = first.clone(); // L1
//     let full = add_suffix(first_clone); // L2
//     println!("{full}, originally {first}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// Which of the following is NOT a kind of undefined behavior?
// Having a pointer to freed memory in a stack frame
// It can be perfectly safe to have a pointer to freed memory in a stack frame.
// The important thing is to not use that pointer again, e.g. by reading it or freeing it.

// All heap data must be owned by exactly one variable.
// Rust deallocates heap data once its owner goes out of scope.
// Ownership can be transferred by moves, which happen on assignments and function calls.
// Heap data can only be accessed through its current owner, not a previous owner.


// In another sense, ownership is a discipline of pointer management.

// References and Borrowing
// Ownership, boxes, and moves provide a foundation for safely programming with the heap.
// However, move-only APIs can be inconvenient to use.



















































