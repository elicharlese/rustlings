// Repetition with Loops
// It’s often useful to execute a block of code more than once.
// For this task, Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning

// Rust has three kinds of loops: loop, while, and for.

// Repeating Code with Loop
// The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// Most terminals support the keyboard shortcut ctrl-c to interrupt a program that is stuck in a continual loop.
// The symbol ^C represents where you pressed ctrl-c .
// You can place the break keyword within the loop to tell the program when to stop executing the loop.
// continue; which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

// Returning values from a loop
// One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job.
// You might also need to pass the result of that operation out of the loop to the rest of your code.
// To do this, you can add the value you want returned after the break expression you use to stop the loop

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
