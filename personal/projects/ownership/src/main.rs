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

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(m1, m2); // L2
//     let s = format!("{} {}", m1, m2); // L3 // Error: m1 and m2 are moved
// }

// fn greet(g1: String, g2: String) {
//     println!("{} {}!", g1, g2); // L1
// }

// calling greet moves the data from m1 and m2 into the parameters of greet.
// Both strings are dropped at the end of greet, and therefore cannot be used within main.

// An alternative greet could return ownership of the strings, like this:

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world"); // L1
//     let (m1_again, m2_again) = greet(m1, m2);
//     let s = format!("{} {}", m1_again, m2_again); // L2
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }

// However, this style of program is quite verbose.
// Rust provides a concise style of reading and writing without moves through references.

// References Are Non-Owning Pointers
// A reference is a kind of pointer.

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world"); // L1
//     greet(&m1, &m2); // L3 // note the ampersands
//     let s = format!("{} {}", m1, m2);
// }

// fn greet(g1: &String, g2: &String) { // note the ampersands
//     println!("{} {}!", g1, g2); // L2
// }

// The expression &m1 uses the ampersand operator to create a reference to (or "borrow") m1.
// The type of the greet parameter g1 is changed to &String, meaning "a reference to a String".
// References are non-owning pointers, because they do not own the data they point to.

// Dereferencing a Pointer Accesses Its Data
// The underlying mechanism is the dereference operator, written with an asterisk (*).

// fn main() {
//     let mut x: Box<i32> = Box::new(1);
//     let a: i32 = *x;         // *x reads the heap value, so a = 1
//     *x += 1;                 // *x on the left-side modifies the heap value,
//                             //     so x points to the value 2

//     let r1: &Box<i32> = &x;  // r1 points to x on the stack
//     let b: i32 = **r1;       // two dereferences get us to the heap value

//     let r2: &i32 = &*x;      // r2 points to the heap value directly
//     let c: i32 = *r2;    // so only one dereference is needed to read it
// }

// Observe the difference between r1 pointing to x on the stack, and r2 pointing to the heap value 2.
// You probably won't see the dereference operator very often when you read Rust code.
// Rust implicitly inserts dereferences and references in certain cases, such as calling a method with the dot operator.

// fn main() {
//     let x: Box<i32> = Box::new(-1);
//     let x_abs1 = i32::abs(*x); // explicit dereference
//     let x_abs2 = x.abs();      // implicit dereference
//     assert_eq!(x_abs1, x_abs2);

//     let r: &Box<i32> = &x;
//     let r_abs1 = i32::abs(**r); // explicit dereference (twice)
//     let r_abs2 = r.abs();       // implicit dereference (twice)
//     assert_eq!(r_abs1, r_abs2);

//     let s = String::from("Hello");
//     let s_len1 = str::len(&s); // explicit reference
//     let s_len2 = s.len();      // implicit reference
//     assert_eq!(s_len1, s_len2);

// }

// For now, the important takeaway is that these conversions are happening with method calls and some macros like println.

// Consider the following program, showing the state of memory after the last line:

// fn main() {
//     let x = Box::new(0);
//     let y = Box::new(&x);
// }

// If you wanted to copy out the number 0 through y, how many dereferences would you need to use? Write your answer as a digit.
// For example, if the correct expression is *y, then the answer is 1.

// ***y is the correct expression. y has the type Box<&Box<i32>>.
// It is a heap pointer to a stack reference to a heap pointer.
// Therefore y must be dereferenced three times for each layer of indirection.

// Rust Avoids Simultaneous Aliasing and Mutation
// Pointers are a powerful and dangerous feature because they enable aliasing.
// Aliasing is accessing the same data through different variables.
// On its own, aliasing is harmless. But combined with mutation, we have a recipe for disaster.
// One variable can "pull the rug out" from another variable in many ways

// - By deallocating the aliased data, leaving the other variable to point to deallocated memory.
// - By mutating the aliased data, invalidating runtime properties expected by the other variable.
// - By concurrently mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.

// As a running example, we are going to look at programs using the vector data structure, Vec.
// Unlike arrays which have a fixed length, vectors have a variable length by storing their elements in the heap.

// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3];
//     vec.push(4);
// }

// The macro vec! creates a vector with the elements between the brackets.
// The vector vec has type Vec<i32>. The syntax <i32> means the elements of the vector have type i32.

// One important implementation detail is that vec allocates a heap array of a certain capacity.
// We can peek into Vec's internals and see this detail for ourselves:

// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3];
// }

// Note: click the binocular icon in the top right of the diagram to toggle this detailed view in any runtime diagram.

// Note: click the binocular icon in the top right of the diagram to toggle this detailed view in any runtime diagram.
//  The vector is at capacity. So when we do a push, the vector has to create a new allocation with larger capacity, copy all the elements over, and deallocate the original heap array.
// So when we do a push, the vector has to create a new allocation with larger capacity, copy all the elements over, and deallocate the original heap array.
// In the diagram above, the array 1 2 3 4 is in a (potentially) different memory location than the original array 1 2 3.

// To tie this back to memory safety, let's bring references into the mix.
// Say we created a reference to a vector's heap data. Then that reference can be invalidated by a push, as simulated below:

// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3];
//     let num: &i32 = &vec[2];
//     vec.push(4);
//     println!("Third element is {}", *num);
// }

// The resize will deallocate the previous array and allocate a new, bigger array.
// In the process, num is left pointing to invalid memory.
// Therefore at L3, dereferencing *num reads invalid memory, causing undefined behavior.

// In more abstract terms, the issue is that the vector vec is both aliased (by the reference num) and mutated (by the operation vec.push(4)).
// So to avoid these kinds of issues, Rust follows a basic principle:

// Pointer Safety Principle: data should never be aliased and mutated at the same time.
// Data can be aliased. Data can be mutated. But data cannot be both aliased and mutated.
// Assigning a box from one variable to another will move ownership, invalidating the previous variable.
// Owned data can only be accessed through the owner — no aliases.

// By design, references are meant to temporarily create aliases.
// In the rest of this section, we will explain the basics of how Rust ensures the safety of references through the borrow checker.

// References Change Permissions on Paths
// The core idea behind the borrow checker is that variables have three kinds of permissions on their data:
// - Read (R): data can be copied to another location.
// - Write (W): data can be mutated in-place.
// - Own (O): data can be moved or dropped.

// These permissions don't exist at runtime, only within the compiler.
// They describe how the compiler "thinks" about your program before the program is executed.

// By default, a variable has read/own permissions (RO) on its data.
// If a variable is annotated with let mut, then it also has the write permission (W).
// The key idea is that references can temporarily remove these permissions.

// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3];
//     let num: &i32 = &vec[2];
//     println!("Third element is {}", *num);
//     vec.push(4);
// }

// fn main() {
//     let x = 0;
//     let mut x_ref = &x;
//     println!("{x_ref} {x}");
// }

// More generally, permissions are defined on paths and not just variables.
// A path is anything you can put on the left-hand side of an assignment.

// Paths include:
// Variables, like a.
// Dereferences of paths, like *a.
// Array accesses of paths, like a[0].
// Fields of paths, like a.0 for tuples or a.field for structs
// Any combination of the above, like *((*a)[0].1).

// Second, why do paths lose permissions when they become unused?
// Because some permissions are mutually exclusive.

// The Borrow Checker Finds Permission Violations
// The goal of these permissions is to ensure that data cannot be mutated if it is aliased.
// Creating a reference to data ("borrowing" it) causes that data to be temporarily read-only until the reference is no longer used.
// Rust uses these permissions in its borrow checker.
// The borrow checker looks for potentially unsafe operations involving references.

// Any time a path is used, Rust expects that path to have certain permissions depending on the operation.
// If you try to compile this program, then the Rust compiler will return the following error:
// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3];
//     let num: &i32 = &vec[2];
//     vec.push(4);
//     println!("Third element is {}", *num);
// }

// Any time a path is used, Rust expects that path to have certain permissions depending on the operation.
// The error message explains that vec cannot be mutated while the reference num is in use.
// That's the surface-level reason — the underlying issue is that num could be invalidated by push.

// Mutable References Provide Unique and Non-Owning Access to Data
// The references we have seen so far are read-only immutable references (also called shared references).
// Immutable references permit aliasing but disallow mutation.
// However, it is also useful to temporarily provide mutable access to data without moving it.

// The mechanism for this is mutable references (also called unique references).

// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut vec[2];
//     *num += 1;
//     println!("Third element is {}", *num);
//     println!("Vector is now {:?}", vec);
// }

// The first observation is what makes mutable references safe.
// Mutable references allow mutation but prevent aliasing.
// The borrowed path vec becomes temporarily unusable, so effectively not an alias.

// The second observation is what makes mutable references useful.
// Mutable references can also be temporarily "downgraded" to read-only references.

// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut vec[2];
//     let num2: &i32 = &*num;
//     println!("{} {}", *num, *num2);
// }

// In this program, the borrow &*num removes the W permission from *num but not the R permission, so println!(..) can read both *num and *num2.

// Permissions Are Returned At The End of a Reference's Lifetime
// We said above that a reference changes permissions while it is "in use".
// The phrase "in use" is describing a reference's lifetime, or the range of code spanning from its birth (where the reference is created) to its death (the last time(s) the reference is used).

// fn main() {
//     let mut x = 1;
//     let y = &x;
//     let z = *y;
//     x += z;
// }

// In the previous examples, a lifetime has been a contiguous region of code.
// However, once we introduce control flow, this is not necessarily the case.

// fn ascii_capitalize(v: &mut Vec<char>) {
//     let c = &v[0];
//     if c.is_ascii_lowercase() {
//         let up = c.to_ascii_uppercase();
//         v[0] = up;
//     } else {
//         println!("Already capitalized: {:?}", v);
//     }
// }

// The variable c has a different lifetime in each branch of the if-statement.
// In the then-block, c is used in the expression c.to_ascii_uppercase().
// In the then-block, c is used in the expression c.to_ascii_uppercase().
// However, in the else-block, c is not used. *v immediately regains the W permission on entry to the else-block.

// Data Must Outlive All Of Its References
// The last safety property is that data must outlive any references to it.

// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     let s_ref = &s;
//     s_ref
// }

// Rust will refuse to compile this program. It will give you a somewhat mysterious error message:

// For now, you can see the underlying safety issue from this simulation:

// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     let s_ref = &s;
//     s_ref
// }

// fn main() {
//     let s_main = return_a_string();
//     println!("{}", s_main);
// }

// fn add_ref(v: &mut Vec<&i32>, n: i32) {
//     let r = &n;
//     v.push(r);
// }

// If this function were allowed, we could call add_ref like this:

fn add_ref(v: &mut Vec<&i32>, n: i32) {
    let r = &n;
    v.push(r);
}

fn main() {
    let mut nums = Vec::new();
    add_ref(&mut nums, 0);
    println!("{}", nums[0]);
}

// At L1, by pushing &n into v, the vector now contains a reference to data within the frame for add_ref.
// However, when add_ref returns, its frame is deallocated.
// Therefore the reference in the vector points to deallocated memory.
// Using the reference by printing v[0] violates memory safety.

// References provide the ability to read and write data without consuming ownership of it.
// References are created with borrows (& and &mut) and used with dereferences (*), often implicitly.

// Rust's borrow checker enforces a system of permissions that ensures references are used safely:
// All variables can read, own, and (optionally) write their data.
// Creating a reference will transfer permissions from the borrowed path to the reference.
// Permissions are returned once the reference's lifetime has ended.
// Data must outlive all references that point to it.
