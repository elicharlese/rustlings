// An Example Program Using Structs
// To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle. 
// We’ll start by using single variables, and then refactor the program until we’re using structs instead.

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// This code succeeds in figuring out the area of the rectangle by calling the area function with each dimension, but we can do more to make this code clear and readable.
// The issue with this code is evident in the signature of area:

// fn area(width: u32, height: u32) -> u32 {
// The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related.
// It would be more readable and more manageable to group width and height together.

// Refactoring with Tuples

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//  Tuples let us add a bit of structure, and we’re now passing just one argument.
// But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.

// This would be even harder for someone else to figure out and keep in mind if they were to use our code.
// Because we haven’t conveyed the meaning of our data in our code, it’s now easier to introduce errors.

// Refactoring with Structs: Adding More Meaning
// We use structs to add meaning by labeling the data.
// We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Here we’ve defined a struct and named it Rectangle.
// Inside the curly brackets, we defined the fields as width and height, both of which have type u32.
// Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance.
// As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it.
// This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1. This is a win for clarity.

// Adding Useful Functionality with Derived Traits
// It’d be useful to be able to print an instance of Rectangle while we’re debugging our program and see the values for all its fields.

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {}", rect1);
// }

// When we compile this code, we get an error with this core message:
// The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption.
// Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display to use with println! and the {} placeholder.

// Let’s try it! The println! macro call will now look like println!("rect1 is {:?}", rect1);. Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.
// The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.
//    = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
//    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

// Compile the code with this change. Drat! We still get an error:
// error[E0277]: `Rectangle` doesn't implement `Debug`

// But again, the compiler gives us a helpful note:
//    = help: the trait `Debug` is not implemented for `Rectangle`
//    = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`

// Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.
// To do that, we add the outer attribute #[derive(Debug)] just before the struct definition

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
// }

// Now when we run the program, we won’t get any errors, and we’ll see the following output:
// Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging.
// When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string.

// Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println! that takes a reference)
// prints the file and line number of where that dbg! macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value.

// Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println! which prints to the standard output console stream (stdout).

// Here’s an example where we’re interested in the value that gets assigned to the width field, as well as the value of the whole struct in rect1:

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }


// We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call. Here’s what the output of this example looks like:
//$ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.61s
//      Running `target/debug/rectangles`
// [src/main.rs:10] 30 * scale = 60
// [src/main.rs:14] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }

// This output uses the pretty Debug formatting of the Rectangle type.
// The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!

// In addition to the Debug trait, Rust has provided a number of traits for us to use with the derive attribute that can add useful behavior to our custom types.
// Those traits and their behaviors are listed in Appendix C.
// There are also many attributes other than derive

// Our area function is very specific: it only computes the area of rectangles.
// It would be helpful to tie this behavior more closely to our Rectangle struct, because it won’t work with any other type.
// Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.

// Which statement best describes a difference between the Display and Debug traits?
// Display is for presenting values to an end-user, while Debug is for developers' internal use


// Method Syntax
// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object)
// their first parameter is always self, which represents the instance of the struct the method is being called on.

// Defining Methods
// Let’s change the area function that has a Rectangle instance as a parameter and instead make an area method defined on the Rectangle struct

// To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle.
// Everything within this impl block will be associated with the Rectangle type.
// Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body.


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// In main, where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the area method on our Rectangle instance.
// The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.
// In the signature for area, we use &self instead of rectangle: &Rectangle.
// The &self is actually short for self: &Self.
// Within an impl block, the type Self is an alias for the type that the impl block is for.
// Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.
// Note that we still need to use the & in front of the self shorthand to indicate this method borrows the Self instance, just as we did in rectangle: &Rectangle.
// Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.

// We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it.
// If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
// Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something
//  else and you want to prevent the caller from using the original instance after the transformation.


// The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization.
// We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

// Note that we can choose to give a method the same name as one of the struct’s fields.

// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }

// Often, but not always, when we give methods with the same name as a field we want it to only return the value in the field and do nothing else.
// Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do.

// Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type’s public API.

// Methods with More Parameters

// Let’s practice using methods by implementing a second method on the Rectangle struct.
// This time, we want an instance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self (the first Rectangle);
// otherwise it should return false.
// That is, once we’ve defined the can_hold method, we want to be able to write the program

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// When we run this code with the main function in Listing 5-14, we’ll get our desired output.
// Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.

// Associated Functions
// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. 
// We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
// We’ve already used one function like this: the String::from function that’s defined on the String type.

// Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
// These are often called new, but new isn’t a special name and isn’t built into the language.

// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

// The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
// To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example.
// This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.

// Multiple impl Blocks
// Each struct is allowed to have multiple impl blocks.

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.
// We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.

// Method Calls are Syntactic Sugar for Function Calls
// Using the concepts we've discussed so far, we can now see how method calls are syntactic sugar for function calls.

// And let's say we have a rectangle r. Then the method calls r.area() and r.set_width(2) are equivalent to this:

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }

//     let mut r = Rectangle { width: 1, height: 2 };

//     let area1 = r.area();
//     let area2 = Rectangle::area(&r);
//     assert_eq!(area1, area2);

//     r.set_width(2);
//     Rectangle::set_width(&mut r, 2);
// }

// The method call r.area() becomes Rectangle::area(&r).
// The function name is the associated function Rectangle::area.
// The function argument is the &self parameter. Rust automatically inserts the borrowing operator &.

// Note: if you are familiar with C or C++, you are used to two different syntaxes for method calls: r.area() and r->area().
// Rust does not have an equivalent to the arrow operator ->.
// Rust will automatically reference and dereference the method receiver when you use the dot operator.

// The method call r.set_width(2) similarly becomes Rectangle::set_width(&mut r, 2).
// This method expects &mut self, so the first argument is a mutable borrow &mut r.
// The second argument is exactly the same, the number 2.

// Rust will insert as many references and dereferences as needed to make the types match up for the self parameter.

// let r = &mut Box::new(Rectangle {
//     width: 1,
//     height: 2
// });
// let area1 = r.area();
// let area2 = Rectangle::area(&**r);
// assert_eq!(area1, area2);

// Rust will add two dereferences (once for the mutable reference, once for the box) and then one immutable borrow because area expects &Rectangle.
// Note that this is also a situation where a mutable reference is "downgraded" into a shared reference.
// Conversely, you would not be allowed to call set_width on a value of type &Rectangle or &Box<Rectangle>.

// Rust does not have a keyword for constructor functions.
// The idiomatic way to define a constructor function is to make an associated function called new, but that is not enforced by the language.

// Methods and Ownership
// methods must be called on structs that have the necessary permissions.
// As a running example, we will use these three methods that take &self, &mut self, and self, respectively.

// impl Rectangle {    
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }

//     fn max(self, other: Rectangle) -> Rectangle {
//         Rectangle { 
//             width: self.width.max(other.width),
//             height: self.height.max(other.height),
//         }
//     }
// }

// Reads and Writes with &self and &mut self
// If we make an owned rectangle with let rect = Rectangle { ... }, then rect has R and O permissions. 
// With those permissions, it is permissible to call the area and max methods:

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {    
//   fn area(&self) -> u32 {
//     self.width * self.height
//   }

//   fn set_width(&mut self, width: u32) {
//     self.width = width;
//   }

//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h
//     }
//   }
// }

// fn main() {
// let rect = Rectangle {
//     width: 0,
//     height: 0
// };

// println!("{}", rect.area());

// let other_rect = Rectangle { width: 1, height: 1 };
// let max_rect = rect.max(other_rect);
// }

// However, if we try to call set_width, we are missing the W permission:

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {    
//   fn area(&self) -> u32 {
//     self.width * self.height
//   }

//   fn set_width(&mut self, width: u32) {
//     self.width = width;
//   }

//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h
//     }
//   }
// }

// fn main() {
// let rect = Rectangle {
//     width: 0,
//     height: 0
// };

// rect.set_width(0);
// }

// We will get a similar error if we try to call set_width on an immutable reference to a Rectangle, even if the underlying rectangle is mutable:

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {    
//   fn area(&self) -> u32 {
//     self.width * self.height
//   }

//   fn set_width(&mut self, width: u32) {
//     self.width = width;
//   }

//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h
//     }
//   }
// }
// fn main() {
// // Added the mut keyword to the let-binding
// let mut rect = Rectangle {
//     width: 0,
//     height: 0
// };
// rect.set_width(1);     // this is now ok

// let rect_ref = &rect;
// rect_ref.set_width(2); // but this is still not ok
// }

// Moves with self
// Calling a method that expects self will move the input struct (unless the struct implements Copy). For example, we cannot use a Rectangle after passing it to max:

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {    
//   fn area(&self) -> u32 {
//     self.width * self.height
//   }

//   fn set_width(&mut self, width: u32) {
//     self.width = width;
//   }

//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h
//     }
//   }
// }
// fn main() {
// let rect = Rectangle {
//     width: 0,
//     height: 0
// };
// let other_rect = Rectangle { 
//     width: 1, 
//     height: 1 
// };
// let max_rect = rect.max(other_rect);
// println!("{}", rect.area());
// }

// Once we call rect.max(..), we move rect and so lose all permissions on it. Trying to compile this program would give us the following error:

// A similar situation arises if we try to call a self method on a reference. For instance, say we tried to make a method set_to_max that assigns self to the output of self.max(..):

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {    
//   fn area(&self) -> u32 {
//     self.width * self.height
//   }

//   fn set_width(&mut self, width: u32) {
//     self.width = width;
//   }

//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h
//     }
//   }
//     fn set_to_max(&mut self, other: Rectangle) {
//         *self = self.max(other);
//     }
// }

// Then we can see that self is missing O permissions in the operation self.max(..). Rust therefore rejects this program with the following error:

// Good Moves and Bad Moves
// You might wonder: why does it matter if we move out of *self?
// In fact, for the case of Rectangle, it actually is safe to move out of *self, even though Rust doesn't let you do it.

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {    
//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h
//     }
//   }

//     fn set_to_max(&mut self, other: Rectangle) {
//         let max = self.max(other);
//         *self = max;
//     }
// }

// fn main() {
//     let mut rect = Rectangle { width: 0, height: 1 };
//     let other_rect = Rectangle { width: 1, height: 0 };
//     rect.set_to_max(other_rect);
// }

// The reason it's safe to move out of *self is because Rectangle does not own any heap data.
// In fact, we can actually get Rust to compile set_to_max by simply adding #[derive(Copy, Clone)] to the definition of Rectangle:

// #[derive(Copy, Clone)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {    
//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h
//     }
//   }
//     fn set_to_max(&mut self, other: Rectangle) {
//         *self = self.max(other);
//     }
// }

// Notice that unlike before, *self now has the O permission. We are allowed to call an owned-self method like max.

// You might wonder: why doesn't Rust automatically derive Copy for Rectangle?
// Rust does not auto-derive Copy for stability across API changes. Imagine that the author of the Rectangle type decided to add a name: String field.
// Then all client code that relies on Rectangle being Copy would suddenly get rejected by the compiler.
// To avoid that issue, API authors must explicitly add #[derive(Copy)] to indicate that they expect their struct to always be Copy.

// To better understand the issue, let's run a simulation. Say we added name: String to Rectangle. What would happen if Rust allowed set_to_max to compile?

// struct Rectangle {
//     width: u32,
//     height: u32,
//     name: String,
// }

// impl Rectangle {    
//   fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Rectangle { 
//       width: w,
//       height: h,
//       name: String::from("max")
//     }
//   }
//     fn set_to_max(&mut self, other: Rectangle) {
//         let max = self.max(other);
//         drop(*self); // This is usually implicit,
//                          // but added here for clarity.
//         *self = max;
//     }
// }

// fn main() {
//     let mut r1 = Rectangle { 
//         width: 9, 
//         height: 9, 
//         name: String::from("r1") 
//     };
//     let r2 = Rectangle {
//         width: 16,
//         height: 16,
//         name: String::from("r2")
//     };
//     r1.set_to_max(r2);
// }

// When max returns, Rust deallocates both strings "r1" and "r2" in the heap.
// Notice the problem: at the location L2, *self is supposed to be readable and writable.
// However, (*self).name (actually r1.name) has been deallocated.

// Therefore when we do *self = max, we encounter undefined behavior.
// When we overwrite *self, Rust will implicitly drop the data previously in *self.
// To make that behavior explicit, we have added drop(*self).
// After calling drop(*self), Rust attempts to free (*self).name a second time.
// That action is a double-free, which is undefined behavior.

// So remember: when you see an error like "cannot move out of *self", that's usually because you're trying to call a self method on a reference like &self or &mut self.
// Rust is protecting you from a double-free.

// Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. 
// In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.


// But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.


// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

// struct Point {
//   x: i32,
//   y: i32
// }
// impl Point {
//   fn get_x(&mut self) -> &mut i32 {
//     &mut self.x
//   }
// }
// fn main() {
//   let mut p = Point { x: 1, y: 2 };
//   let x = p.get_x();
//   *x += 1;
//   println!("{} {}", *x, p.y);
// }

// This program does not compile.
// Because get_x mutably borrows all of p, a program cannot use p in any way until x is no longer used. Therefore reading x and p.y in the same line is an ownership error.























