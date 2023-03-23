// Enums and Pattern Matching
// Enums allow you to define a type by enumerating its possible variants.
// First, we’ll define and use an enum to show how an enum can encode meaning along with data.
// Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing.
// Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum.
// Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.

// Defining an Enum
// Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values.
// Say we need to work with IP addresses.
// Currently, two major standards are used for IP addresses: version four and version six.
// Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants, which is where enumeration gets its name.

// Any IP address can be either a version four or a version six address, but not both at the same time.
// That property of IP addresses makes the enum data structure appropriate, because an enum value can only be one of its variants.
// Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

// We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6. These are the variants of the enum:

// enum IpAddrKind {
//     V4,
//     V6,
// }

// IpAddrKind is now a custom data type that we can use elsewhere in our code.

// Enum Values
// We can create instances of each of the two variants of IpAddrKind like this:

// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;

// Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
// This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. We can then, for instance, define a function that takes any IpAddrKind:

// fn route(ip_kind: IpAddrKind) {}

// And we can call this function with either variant:

// route(IpAddrKind::V4);
// route(IpAddrKind::V6);

// Thinking more about our IP address type, at the moment we don’t have a way to store the actual IP address data; we only know what kind it is.
// Given that you just learned about structs in Chapter 5, you might be tempted to tackle this problem with structs

// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// We’ve used a struct to bundle the kind and address values together, so now the variant is associated with the value.
// However, representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant.
// This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:

// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// We attach data to each variant of the enum directly, so there is no need for an extra struct.
// Here it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
// We automatically get this constructor function defined as a result of defining the enum.

// There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
// Version four type IP addresses will always have four numeric components that will have values between 0 and 255.

// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// However, as it turns out, wanting to store IP addresses and encode which kind they are is so common that the standard library has a definition we can use!
// Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used, but it embeds the address data inside 
// the variants in the form of two different structs, which are defined differently for each variant:

// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
// You can even include another enum! Also, standard library types are often not much more complicated than what you might come up with.

// Let’s look at another example of an enum in Listing 6-2: this one has a wide variety of types embedded in its variants.

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// This enum has four variants with different types:
// - Quit has no data associated with it at all.
// - Move has named fields like a struct does.
// - Write includes a single String.
// - ChangeColor includes three i32 values.

// Defining an enum with variants such as the ones in Listing 6-2 is similar to defining different kinds of struct definitions, except
//  the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type.
// The following structs could hold the same data that the preceding enum variants hold:

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }

// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of
// these kinds of messages as we could with the Message enum defined in Listing 6-2, which is a single type.

// There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.
// Here’s a method named call that we could define on our Message enum:

// fn main() {
//     impl Message {
//         fn call(&self) {
//             // method body would be defined here
//         }
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// The body of the method would use self to get the value that we called the method on.
// In this example, we’ve created a variable m that has the value Message::Write(String::from("hello")), and that is what self will be in the body of the call method when m.call() runs.

// The Option Enum and Its Advantages Over Null Values
// The Option type encodes the very common scenario in which a value could be something or it could be nothing.
// Option, which is another enum defined by the standard library.

// Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that
//  are extremely common in other programming languages.

// Rust doesn’t have the null feature that many other languages have.
// Null is a value that means there is no value there.
// In languages with null, variables can always be in one of two states: null or not-null.

// The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind.
// Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

//  As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.
// This enum is Option<T>, and it is defined by the standard library as follows:

// enum Option<T> {
//     None,
//     Some(T),
// }

// The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
// Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix.
// The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

// or now, all you need to know is that <T> means the Some variant of the Option enum can hold one piece of data of any type,
//  and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.
// Here are some examples of using Option values to hold number types and string types:

// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('e');

//     let absent_number: Option<i32> = None;
// }

// The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a different type.
// Rust can infer these types because we’ve specified a value inside the Some variant.
// Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
// In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value.

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x + y;
// }

// Intense! In effect, this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.
// When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value.
// We can proceed confidently without having to check for null before using that value.
// Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

// In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
// Eliminating the risk of incorrectly assuming a not-null value helps you to be more confident in your code.
// In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>.
// Then, when you use that value, you are required to explicitly handle the case when the value is null.
// Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
// This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

// The Option<T> enum has a large number of methods that are useful in a variety of situations; you can check them out in its documentation.
// In general, in order to use an Option<T> value, you want to have code that will handle each variant.
// You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T.
// You want some other code to run if you have a None value, and that code doesn’t have a T value available.
// The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which
// variant of the enum it has, and that code can use the data inside the matching value.

// Consider these two representations of a Result type that contains a value T if a computation succeeds, or an error E if it fails.

// struct Result1<T, E> {
//     ok: Option<T>,
//     err: Option<E>,
// }
// enum Result2<T, E> {
//     Ok(T),
//     Err(E)
// }

// The enum Result2 is considered more idiomatic than the struct Result1 in Rust. Which statement below is NOT a valid reason why?
// The struct contains Option types, which are only intended to wrap structs
// It's perfectly fine to have structs contain Option types as fields. But if your data structure has invariants like "exactly one of
// two optional fields should be Some", then that invariant is better ensured by use of an enum.

// The match Control Flow Construct
// Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
// Patterns can be made up of literal values, variable names, wildcards, and many other things
// The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

// Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into.
// In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

// Speaking of coins, let’s use them as an example using match!
// We can write a function that takes an unknown United States coin and, in a similar way as the counting machine, determines which coin it is and return its value in cents.

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// Let’s break down the match in the value_in_cents function.
// First, we list the match keyword followed by an expression, which in this case is the value coin.
// This seems very similar to an expression used with if, but there’s a big difference: with if, the expression needs to return a Boolean value, but here, it can return any type.
// The type of coin in this example is the Coin enum that we defined on the first line.

// Next are the match arms. An arm has two parts: a pattern and some code.
// The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run.
// Each arm is separated from the next with a comma.

// When the match expression executes, it compares the resulting value against the pattern of each arm, in order.
// If a pattern matches the value, the code associated with that pattern is executed.
// If that pattern doesn’t match the value, execution continues to the next arm, much as in a coin-sorting machine.
// The code associated with each arm is an expression, and the resulting value of the expression in the matching arm
// is the value that gets returned for the entire match expression.

// If you want to run multiple lines of code in a match arm, you must use curly brackets, and the comma following the arm is then optional.

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// Patterns that Bind to Values
// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
// This is how we can extract values out of enum variants.

// No other coins got state designs, so only quarters have this extra value.
// We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// In the match expression for this code, we add a variable called state to the pattern that matches values of the variant Coin::Quarter.
// When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state.
// Then we can use state in the code for that arm, like so:

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// When we compare that value with each of the match arms, none of them match until we reach Coin::Quarter(state).
// At that point, the binding for state will be the value UsState::Alaska. 
// We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.

// Matching with Option<T>
// Instead of comparing coins, we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.
// Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value.
// If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// let five = Some(5);
// let six = plus_one(five);
// let none = plus_one(None);

// Let’s examine the first execution of plus_one in more detail.
// When we call plus_one(five), the variable x in the body of plus_one will have the value Some(5).
// We then compare that against each match arm.
//            None => None,

// The Some(5) value doesn’t match the pattern None, so we continue to the next arm.
//            Some(i) => Some(i + 1),

// The i binds to the value contained in Some, so i takes the value 5.
// The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.
// Now let’s consider the second call of plus_one in Listing 6-5, where x is None.
// We enter the match and compare to the first arm.
//             None => None,

// There’s no value to add to, so the program stops and returns the None value on the right side of =>.
// Because the first arm matched, no other arms are compared.

// You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it.

// Matches Are Exhaustive
// There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities.
// Consider this version of our plus_one function, which has a bug and won’t compile:

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

// We didn’t handle the None case, so this code will cause a bug.
// Luckily, it’s a bug Rust knows how to catch. If we try to compile this code, we’ll get this error:

// Rust knows that we didn’t cover every possible case and even knows which pattern we forgot!
// Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.
// Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None
// case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier impossible.

// Catch-all Patterns and the _ Placeholder
// Using enums, we can also take special actions for a few particular values, but for all other values take one default action.
// Here’s a match that implements that logic, with the result of the dice roll hardcoded rather than a random value, and all other logic represented
// by functions without bodies because actually implementing them is out of scope for this example:

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {}
// }

// The code that runs for the other arm uses the variable by passing it to the move_player function.
// This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed.
// This catch-all pattern meets the requirement that match must be exhaustive.
// Note that we have to put the catch-all arm last because the patterns are evaluated in order.
// If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!

// Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern:
// _ is a special pattern that matches any value and does not bind to that value.
// This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.


// Let’s change the rules of the game: now, if you roll anything other than a 3 or a 7, you must roll again.
// We no longer need to use the catch-all value, so we can change our code to use _ instead of the variable named other:

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}
// }

// This example also meets the exhaustiveness requirement because we’re explicitly ignoring all other values in the last arm; we haven’t forgotten anything.
// Finally, we’ll change the rules of the game one more time, so that nothing else happens on your turn if you roll anything other than a 3 or a 7.
// We can express that by using the unit value (the empty tuple type we mentioned in “The Tuple Type” section) as the code that goes with the _ arm:

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => (),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
// }

// Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this case.

// How Matches Interact with Ownership
// If an enum contains non-copyable data like a String, then you should be careful with whether a match will move or borrow that data.

// fn main() {
//     let opt: Option<String> = 
//         Some(String::from("Hello world"));

//     match opt {
//         Some(_) => println!("Some!"),
//         None => println!("None!")
//     };

//     println!("{:?}", opt);
//  }

// But if we replace the placeholder in Some(_) with a variable name, like Some(s), then the program will NOT compile:

// fn main() {
//     let opt: Option<String> = 
//         Some(String::from("Hello world"));

//     match opt {
//         // _ became s
//         Some(s) => println!("Some: {}", s),
//         None => println!("None!")
//     };

//     println!("{:?}", opt);
// }

// opt is a plain enum — its type is Option<String> and not a reference like &Option<String>. Therefore a match on opt will move non-ignored fields like s. 
// Notice how opt loses read and own permission sooner in the second program compared to the first.
// After the match expression, the data within opt has been moved, so it is illegal to read opt in the println.

// If we want to peek into opt without moving its contents, the idiomatic solution is to match on a reference:

// fn main() {
//     let opt: Option<String> = 
//         Some(String::from("Hello world"));

//     // opt became &opt
//     match &opt {
//         Some(s) => println!("Some: {}", s),
//         None => println!("None!")
//     };

//     println!("{:?}", opt);
// }

// Rust will “push down” the reference from the outer enum, &Option<String>, to the inner field, &String.
// Therefore s has type &String, and opt can be used after the match.
// To better understand this “pushing down” mechanism, see the section about binding modes in the Rust Reference.

// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

// enum Location {
//     Point(i32),
//     Range(i32, i32)
// }

// fn main() {
//     let l: Location = Location::Range(0, 5);
//     let n = match l {
//         Location::Point(_) => -1,
//         Location::Range(_, n) => n,
//         Location::Range(0, _) => 0,
//         _ => -2
//     };
//     println!("{n}");
// }

// The output of this program will be: 5
// Each match is tried from top to bottom. Both the second and third pattern are applicable, so the second one is used.

// Consider this method implemented for the Option type:

// impl<T> Option<T> {
//     fn unwrap_or(self, other: T) -> T {
//         match self {
//             Some(t) => t,
//             None => other
//         }
//     }
// }

// Which sentence best describes the behavior of this function?
// Returns the object inside self if it exists, and other otherwise
// This function "unwraps" the option by consuming ownership of it and retrieving the value inside, but if no value exists then it falls back by returning other.
// This is a real function in the standard library!

//Determine whether the program will pass the compiler.
//If it passes, write the expected output of the program if it were executed.

// #[derive(Debug)]
// enum Either {
//     Left(usize),
//     Right(String)
// }
// fn main() {
//     let x = Either::Right(String::from("Hello world"));
//     let value = match x {
//         Either::Left(n) => n,
//         Either::Right(s) => s.len()
//     };
//     println!("{x:?} {value}");
// }

// The match arm Either::Right(s) moves the field s, so x cannot be used in the println.

// Consider these two implementations of a function to decrement an unsigned number twice.

// fn decr_twice_v1(n: u32) -> Option<u32> {
//     match n {
//         0 => None,
//         1 => None,
//         n2 => Some(n2 - 2)
//     }
// }
// fn decr_twice_v2(n: u32) -> Option<u32> {
//     if n == 0 {
//         None
//     } else if n == 1 {
//         None
//     } else {
//         Some(n - 2)
//     }
// }

// The functions have the same behavior for: All inputs
// The match and if perform the same function here. A match is like a specialized if that checks for equality of the matched object.

// Concise Control Flow with if let
// The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
// Consider the program in Listing 6-6 that matches on an Option<u8> value in the config_max variable but only wants to execute code if the value is the Some variant.

// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {}", max),
//         _ => (),
//     }
// }

// If the value is Some, we print out the value in the Some variant by binding the value to the variable max in the pattern.
// We don’t want to do anything with the None value.
// To satisfy the match expression, we have to add _ => () after processing just one variant, which is annoying boilerplate code to add.

// Instead, we could write this in a shorter way using if let.
// The following code behaves the same as the match in Listing 6-6:

// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max);
//     }
// }

// The syntax if let takes a pattern and an expression separated by an equal sign.
// It works the same way as a match, where the expression is given to the match and the pattern is its first arm.
// In this case, the pattern is Some(max), and the max binds to the value inside the Some.
// We can then use max in the body of the if let block in the same way as we used max in the corresponding match arm.
// The code in the if let block isn’t run if the value doesn’t match the pattern.

// Using if let means less typing, less indentation, and less boilerplate code.
// However, you lose the exhaustive checking that match enforces.
// Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

// In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
// We can include an else with an if let.
// The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else
// Recall the Coin enum definition in Listing 6-4, where the Quarter variant also held a UsState value.
// If we wanted to count all non-quarter coins we see while also announcing the state of the quarters, we could do that with a match expression like this:

// fn main() {
//     let mut count = 0;
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1,
//     }
// }

// Or we could use an if let and else expression like this:

// fn main() {
//     let mut count = 0;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         count += 1;
//     }
// }

// If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in your Rust toolbox as well.

// Which control flow construct would be most idiomatic to use in the following function?

// enum Location {
//     Point(i32),
//     Range(i32, i32)
// }

// fn print_range_max(loc: &Location) {
//   // print the second field of Range, if loc is a Range
// }

// If the function only has an effect in one condition, an if let is most idiomatic.

// Which control flow construct would be most idiomatic to use in the following function?

// enum Location {
//     Point(i32),
//     Range(i32, i32)
// }

// fn get_start(loc: &Location) -> i32 {
//   // return the first field of Range or the only field of Point
// }

// If the function needs to return a value for each condition, then a match is most appropriate.

// We’ve shown how the standard library’s Option<T> type helps you use the type system to prevent errors.
// When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.
// Creating custom types to use in your API ensures type safety: the compiler will make certain your functions get only values of the type each function expects.
// In order to provide a well-organized API to your users that is straightforward to use and only exposes exactly what your users will need, let’s now turn to Rust’s modules.

// Ownership Inventory #1
// The Ownership Inventory is a series of quizzes that check your understanding of ownership in real-world scenarios.

// A new technology: the in-browser IDE
// The IDE lets you get information about unfamiliar functions and types.
// For example, try doing the following actions in the program below:
// - Hover your mouse over replace to see its type and description.
// - Hover your mouse over s2 to see its inferred type.

// Question 1
// Program 1:

/// Makes a string to separate lines of text,
/// returning a default if the provided string is blank
// fn main() {
//     make_separator("Hello World"::&str);
// }

// fn make_separator(user_str: &str) -> &str {
//     if user_str == "" {
//         let default = "=".repeat(10);
//         &default
//     } else {
//         user_str
//     }
// }

// If you tried to compile this function, which of the following best describes the compiler error you would get?

// Answer:
// cannot return reference to local variable default
// Because default lives on the stack within make_separator, it will be deallocated once a call to make_separator ends. This leaves &default pointing to deallocated memory.
// Rust therefore complains that you cannot return a reference to a local variable.


// Question 2
// Program 1:

/// Makes a string to separate lines of text,
/// returning a default if the provided string is blank
// fn make_separator(user_str: &str) -> &str {
//     if user_str == "" {
//         let default = "=".repeat(10);
//         &default
//     } else {
//         user_str
//     }
// }

// Normally if you try to compile this function, the compiler returns the following error:

// error[E0515]: cannot return reference to local variable `default`
//  --> test.rs:6:9
//   |
// 6 |         &default
//   |         ^^^^^^^^ returns a reference to data owned by the current function

// Assume that the compiler did NOT reject this function.
// Which (if any) of the following programs would (1) pass the compiler, and (2) possibly cause undefined behavior if executed?
// Check each program that satisfies both criteria, OR check "None of these programs" if none are satisfying.

// Answer:
// let s = make_separator("");
// println!("{s}");
// First, the caller must pass an empty string to trigger the problematic if-condition.
// This returns a dangling pointer. Second, the caller must use the result of make_separator, e.g. via println.

// Question 3
// Program 1:

/// Makes a string to separate lines of text,
/// returning a default if the provided string is blank
// fn make_separator(user_str: &str) -> &str {
//     if user_str == "" {
//         let default = "=".repeat(10);
//         &default
//     } else {
//         user_str
//     }
// }

// Of the following fixes (highlighted in yellow), which fix best satisfies these three criteria:

// The fixed function passes the Rust compiler,
// The fixed function preserves the intention of the original code, and
// The fixed function does not introduce unnecessary inefficiencies

// Answer:
// fn make_separator(user_str: &str) -> String {
//     if user_str == "" {
//         let default = "=".repeat(10);
//         default
//     } else {
//         user_str.to_string()
//     }
// }

// There is no valid way to return a pointer to a stack-allocated variable. 
// The simple solution is therefore to change the return type to String and copy the input user_str into an owned string. 
// However, requiring user_str to be a String would reduce the flexibility of the API, e.g. a caller could not call make_separator on a substring of a bigger string. 
// It would also require callers to heap-allocate strings, e.g. they could not use a string literal like make_separator("Rust").

// The most idiomatic solution to this problem uses a construct you haven't seen yet: Cow. 
// The copy-on-write smart pointer would enable this function to return either an owned string or a string reference without a type error.


// Question 4
// Program 2:

/// Gets the string out of an option if it exists,
/// returning a default otherwise
// fn get_or_default(arg: &Option<String>) -> String {
//     if arg.is_none() {
//         return String::new();
//     }
//     let s = arg.unwrap();
//     s.clone()
// }

// If you tried to compile this function, which of the following best describes the compiler error you would get?

// Answer:
// cannot move out of arg in arg.unwrap()
// The function Option::unwrap expects self, meaning it expects ownership of arg.
// However arg is an immutable reference to an option, so it cannot provide ownership of the option.
// Therefore the compiler complains that we cannot move out of arg via unwrap.

// Question 5
// Program 2:

/// Gets the string out of an option if it exists,
/// returning a default otherwise
// fn get_or_default(arg: &Option<String>) -> String {
//     if arg.is_none() {
//         return String::new();
//     }
//     let s = arg.unwrap();
//     s.clone()
// }

// Normally if you try to compile this function, the compiler returns the following error:

// error[E0507]: cannot move out of `*arg` which is behind a shared reference
//    --> test.rs:7:13
//     |
// 7   |     let s = arg.unwrap();
//     |             ^^^^--------
//     |             |   |
//     |             |   `*arg` moved due to this method call
//     |             help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
//     |             move occurs because `*arg` has type `Option<String>`, which does not implement the `Copy` trait

// Assume that the compiler did NOT reject this function.
// Which (if any) of the following programs would (1) pass the compiler, and (2) possibly cause undefined behavior if executed? 
// Check each program that satisfies both criteria, OR check "None of these programs" if none are satisfying.

// Question 6
// Program 2:

/// Gets the string out of an option if it exists,
/// returning a default otherwise
// fn get_or_default(arg: &Option<String>) -> String {
//     if arg.is_none() {
//         return String::new();
//     }
//     let s = arg.unwrap();
//     s.clone()
// }
// Of the following fixes (highlighted in yellow), which fix best satisfies these three criteria:

// The fixed function passes the Rust compiler,
// The fixed function preserves the intention of the original code, and
// The fixed function does not introduce unnecessary inefficiencies





