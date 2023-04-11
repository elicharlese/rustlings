// Patterns and Matching
// Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple.
// Using patterns in conjunction with match expressions and other constructs gives you more control over a program’s control flow.
// A pattern consists of some combination of the following:

// Literals
// Destructured arrays, enums, structs, or tuples
// Variables
// Wildcards
// Placeholders
// Some example patterns include x, (a, 3), and Some(Color::Red).
// In the contexts in which patterns are valid, these components describe the shape of data.
// Our program then matches values against the patterns to determine whether it has the correct shape of data to continue running a particular piece of code.

// To use a pattern, we compare it to some value.
// If the pattern matches the value, we use the value parts in our code.
// Recall the match expressions in Chapter 6 that used patterns, such as the coin-sorting machine example.
// If the value fits the shape of the pattern, we can use the named pieces. If it doesn’t, the code associated with the pattern won’t run.

// This chapter is a reference on all things related to patterns. We’ll cover the valid places to use patterns, the difference between refutable and irrefutable patterns, and the different kinds of pattern syntax that you might see.
// By the end of the chapter, you’ll know how to use patterns to express many concepts in a clear way.

All the Places Patterns Can Be Used
Patterns pop up in a number of places in Rust, and you’ve been using them a lot without realizing it! This section discusses all the places where patterns are valid.

match Arms
As discussed in Chapter 6, we use patterns in the arms of match expressions. Formally, match expressions are defined as the keyword match, a value to match on, and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern, like this:

match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
For example, here's the match expression from Listing 6-5 that matches on an Option<i32> value in the variable x:

match x {
    None => None,
    Some(i) => Some(i + 1),
}
The patterns in this match expression are the None and Some(i) on the left of each arrow.

One requirement for match expressions is that they need to be exhaustive in the sense that all possibilities for the value in the match expression must be accounted for. One way to ensure you’ve covered every possibility is to have a catchall pattern for the last arm: for example, a variable name matching any value can never fail and thus covers every remaining case.

The particular pattern _ will match anything, but it never binds to a variable, so it’s often used in the last match arm. The _ pattern can be useful when you want to ignore any value not specified, for example. We’ll cover the _ pattern in more detail in the “Ignoring Values in a Pattern” section later in this chapter.

Conditional if let Expressions
In Chapter 6 we discussed how to use if let expressions mainly as a shorter way to write the equivalent of a match that only matches one case. Optionally, if let can have a corresponding else containing code to run if the pattern in the if let doesn’t match.

Listing 18-1 shows that it’s also possible to mix and match if let, else if, and else if let expressions. Doing so gives us more flexibility than a match expression in which we can express only one value to compare with the patterns. Also, Rust doesn't require that the conditions in a series of if let, else if, else if let arms relate to each other.

The code in Listing 18-1 determines what color to make your background based on a series of checks for several conditions. For this example, we’ve created variables with hardcoded values that a real program might receive from user input.

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

If the user specifies a favorite color, that color is used as the background. If no favorite color is specified and today is Tuesday, the background color is green. Otherwise, if the user specifies their age as a string and we can parse it as a number successfully, the color is either purple or orange depending on the value of the number. If none of these conditions apply, the background color is blue.

This conditional structure lets us support complex requirements. With the hardcoded values we have here, this example will print Using purple as the background color.

You can see that if let can also introduce shadowed variables in the same way that match arms can: the line if let Ok(age) = age introduces a new shadowed age variable that contains the value inside the Ok variant. This means we need to place the if age > 30 condition within that block: we can’t combine these two conditions into if let Ok(age) = age && age > 30. The shadowed age we want to compare to 30 isn’t valid until the new scope starts with the curly bracket.

The downside of using if let expressions is that the compiler doesn’t check for exhaustiveness, whereas with match expressions it does. If we omitted the last else block and therefore missed handling some cases, the compiler would not alert us to the possible logic bug.

while let Conditional Loops
Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match. In Listing 18-2 we code a while let loop that uses a vector as a stack and prints the values in the vector in the opposite order in which they were pushed.

let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}

This example prints 3, 2, and then 1. The pop method takes the last element out of the vector and returns Some(value). If the vector is empty, pop returns None. The while loop continues running the code in its block as long as pop returns Some. When pop returns None, the loop stops. We can use while let to pop every element off our stack.

for Loops
In a for loop, the value that directly follows the keyword for is a pattern. For example, in for x in y the x is the pattern. Listing 18-3 demonstrates how to use a pattern in a for loop to destructure, or break apart, a tuple as part of the for loop.

let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}

// $ cargo run
//    Compiling patterns v0.1.0 (file:///projects/patterns)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target/debug/patterns`
// a is at index 0
// b is at index 1
// c is at index 2

We adapt an iterator using the enumerate method so it produces a value and the index for that value, placed into a tuple. The first value produced is the tuple (0, 'a'). When this value is matched to the pattern (index, value), index will be 0 and value will be 'a', printing the first line of the output.

let Statements
Prior to this chapter, we had only explicitly discussed using patterns with match and if let, but in fact, we’ve used patterns in other places as well, including in let statements. For example, consider this straightforward variable assignment with let:

let x = 5;

Every time you've used a let statement like this you've been using patterns, although you might not have realized it! More formally, a let statement looks like this:

let PATTERN = EXPRESSION;

In statements like let x = 5; with a variable name in the PATTERN slot, the variable name is just a particularly simple form of a pattern. Rust compares the expression against the pattern and assigns any names it finds. So in the let x = 5; example, x is a pattern that means “bind what matches here to the variable x.” Because the name x is the whole pattern, this pattern effectively means “bind everything to the variable x, whatever the value is.”

To see the pattern matching aspect of let more clearly, consider Listing 18-4, which uses a pattern with let to destructure a tuple.

let (x, y, z) = (1, 2, 3);

Here, we match a tuple against a pattern. Rust compares the value (1, 2, 3) to the pattern (x, y, z) and sees that the value matches the pattern, so Rust binds 1 to x, 2 to y, and 3 to z. You can think of this tuple pattern as nesting three individual variable patterns inside it.

If the number of elements in the pattern doesn’t match the number of elements in the tuple, the overall type won’t match and we’ll get a compiler error. For example, Listing 18-5 shows an attempt to destructure a tuple with three elements into two variables, which won’t work.

let (x, y) = (1, 2, 3);

Attempting to compile this code results in this type error:

// $ cargo run
//    Compiling patterns v0.1.0 (file:///projects/patterns)
// error[E0308]: mismatched types
//  --> src/main.rs:2:9
//   |
// 2 |     let (x, y) = (1, 2, 3);
//   |         ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
//   |         |
//   |         expected a tuple with 3 elements, found one with 2 elements
//   |
//   = note: expected tuple `({integer}, {integer}, {integer})`
//              found tuple `(_, _)`

// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `patterns` due to previous error

To fix the error, we could ignore one or more of the values in the tuple using _ or .., as you’ll see in the “Ignoring Values in a Pattern” section. If the problem is that we have too many variables in the pattern, the solution is to make the types match by removing variables so the number of variables equals the number of elements in the tuple.

Function Parameters
Function parameters can also be patterns. The code in Listing 18-6, which declares a function named foo that takes one parameter named x of type i32, should by now look familiar.

fn foo(x: i32) {
    // code goes here
}

The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the pattern. Listing 18-7 splits the values in a tuple as we pass it to a function.

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

This code prints Current location: (3, 5). The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5.

We can also use patterns in closure parameter lists in the same way as in function parameter lists, because closures are similar to functions, as discussed in Chapter 13.

At this point, you’ve seen several ways of using patterns, but patterns don’t work the same in every place we can use them. In some places, the patterns must be irrefutable; in other circumstances, they can be refutable. We’ll discuss these two concepts next.

Question 1
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

fn main() {
    let mut v = vec![(1, 2), (3, 4)].into_iter();
    let mut sum = 0;
    while let Some(t) = v.next() {
        let (_, n) = t;
        sum += n;
    }
    println!("{sum}");
}

This program does compile.

The output of this program will be: 6
This example provides a valid use of while let matching and let matching. Note that you could combine them, e.g.

while let Some((_, n)) = v.next() {
    /* ... */
}

Refutability: Whether a Pattern Might Fail to Match
Patterns come in two forms: refutable and irrefutable. Patterns that will match
for any possible value passed are irrefutable. An example would be x in the
statement let x = 5; because x matches anything and therefore cannot fail
to match. Patterns that can fail to match for some possible value are
refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than
Some, the Some(x) pattern will not match.
Function parameters, let statements, and for loops can only accept
irrefutable patterns, because the program cannot do anything meaningful when
values don’t match. The if let and while let expressions accept
refutable and irrefutable patterns, but the compiler warns against
irrefutable patterns because by definition they’re intended to handle possible
failure: the functionality of a conditional is in its ability to perform
differently depending on success or failure.
In general, you shouldn’t have to worry about the distinction between refutable
and irrefutable patterns; however, you do need to be familiar with the concept
of refutability so you can respond when you see it in an error message. In
those cases, you’ll need to change either the pattern or the construct you’re
using the pattern with, depending on the intended behavior of the code.
Let’s look at an example of what happens when we try to use a refutable pattern
where Rust requires an irrefutable pattern and vice versa. Listing 18-8 shows a
let statement, but for the pattern we’ve specified Some(x), a refutable
pattern. As you might expect, this code will not compile.
fn main() {
    let some_option_value: Option<i32> = None;
    let Some(x) = some_option_value;
}

If some_option_value was a None value, it would fail to match the pattern
Some(x), meaning the pattern is refutable. However, the let statement can
only accept an irrefutable pattern because there is nothing valid the code can
do with a None value. At compile time, Rust will complain that we’ve tried to
use a refutable pattern where an irrefutable pattern is required:

// $ cargo run
//    Compiling patterns v0.1.0 (file:///projects/patterns)
// error[E0005]: refutable pattern in local binding: `None` not covered
//    --> src/main.rs:3:9
//     |
// 3   |     let Some(x) = some_option_value;
//     |         ^^^^^^^ pattern `None` not covered
//     |
//     = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
//     = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
// note: `Option<i32>` defined here
//     = note: the matched value is of type `Option<i32>`
// help: you might want to use `if let` to ignore the variant that isn't matched
//     |
// 3   |     let x = if let Some(x) = some_option_value { x } else { todo!() };
//     |     ++++++++++                                 ++++++++++++++++++++++

// For more information about this error, try `rustc --explain E0005`.
// error: could not compile `patterns` due to previous error

Because we didn’t cover (and couldn’t cover!) every valid value with the pattern Some(x), Rust rightfully produces a compiler error.

If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: instead of using let, we can use if let. Then if the pattern doesn’t match, the code will just skip the code in the curly brackets, giving it a way to continue validly. Listing 18-9 shows how to fix the code in Listing 18-8.

if let Some(x) = some_option_value {
    println!("{}", x);
}

Rust complains that it doesn’t make sense to use if let with an irrefutable pattern:

// $ cargo run
//    Compiling patterns v0.1.0 (file:///projects/patterns)
// warning: irrefutable `if let` pattern
//  --> src/main.rs:2:8
//   |
// 2 |     if let x = 5 {
//   |        ^^^^^^^^^
//   |
//   = note: `#[warn(irrefutable_let_patterns)]` on by default
//   = note: this pattern will always match, so the `if let` is useless
//   = help: consider replacing the `if let` with a `let`

// warning: `patterns` (bin "patterns") generated 1 warning
//     Finished dev [unoptimized + debuginfo] target(s) in 0.39s
//      Running `target/debug/patterns`
// 5

For this reason, match arms must use refutable patterns, except for the last arm, which should match any remaining values with an irrefutable pattern. Rust allows us to use an irrefutable pattern in a match with only one arm, but this syntax isn’t particularly useful and could be replaced with a simpler let statement.

Now that you know where to use patterns and the difference between refutable and irrefutable patterns, let’s cover all the syntax we can use to create patterns.

Question 1
Consider pattern-matching on an expression of some type T. Which of these statements best describes the difference between a refutable and an irrefutable pattern?
Refutable patterns do not match some value of type T, while irrefutable patterns match all values of type T
A pattern is refutable if there exists some value of the expected type that isn't matched by the pattern.

Question 2
Consider the following program:

let x: &[(i32, i32)] = &[(0, 1)];
Which of the following are refutable patterns for x?
&[(x, y)]

&[(x, y), ..]








