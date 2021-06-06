// ------------- constant ----------------------
// Constants aren’t just immutable by default they’re always immutable.
// We declare constants using the const keyword instead of the let keyword,
// and the type of the value must be annotated.
// Constants can be declared in any scope, including the global scope.

// fn main () {

// const PI:f32 = 3.14;
// println!("{}",PI);
// }

// ------------- variable ----------------------

// fn main() {

// let mut x = 5;
// println!("The value of x is: {}",x);
// x = 6;
// println!("The value of x is: {}",x);

// }

// --------------- shadowing ---------------------

// fn main () {
//     let y = 10;
//     let y = y + 5;
//     let y = y * 5;
//     println!("The value is: {}",y);

//     let space = "hafsa shaikh";
//     let space = space.len();
//     println!("{}",space);

// }

// ---------------- DATA TYPES ----------------------
// Rust is a statically typed language, which means that it must know the types of all variables at compile time.

// 1- Scalar      2- Compound

// A scalar type represents a single value. Rust has four primary scalar types:
// integers, floating-point numbers, Booleans, and characters.
// Compound type can group multiple values into one type. Rust has two primitive compound types.
// 1- Tuples  2- Arrays

// A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// Contain heterogeneous data.
// We can access a tuple element directly by using a period (.) followed by the index of the value we want to access.

// fn main () {
//     let tup = ("hafsa",20, 30.0,'A');
//     // let value = tup.1;
//     println!("{:#?}",tup);
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {}", y);
// }

// Another way to have a collection of multiple values is with an array.
// Contains homogenous data.
// Every element of an array must have the same type not like a Tuple.
// We can access elements of an array using indexing in square bracket.
// An array isn’t flexible and not allowed to grow or shrink in size.

// fn main () {
//     let array = [1,2,3,4,5];
//     println!("{:?}",array[4]);
// }

// fn main () {
//     let aray = [5; 10];
//     println!("{:#?}",aray);

// }

// --------------- Function --------------------------

// fn main () {
//   other_function()
// }
// fn other_function () {
//     println!("Hello World");
// }

// fn main () {
//  param(20,20.2);
// }
// fn param (x:i32, y:f64) {
//   println!("The value of x is: {} and y is: {}",x,y);
// }

// fn main () {
//     let x = 4;
//     let y = {
//         let x = 5;
//         x + 4           // This is an expression
//     };

//     println!("The value of x is {}",x);
//     println!("The value of y is {}",y);
// }

// fn main () {
//     let x = five();
//    println!("{}",x);
// }

// fn five () -> i32 {
//     5
// }

// fn main () {
//     let x = five(20);
//    println!("{}",x);
// }

// fn five (y: i32) -> i32 {
//     y * 2
// }

// --------------- Control Flow --------------------------- 

// ** if expression **
// An if expression allows you to branch your code depending on conditions. You provide a
// condition and then state, “If this condition is met, run this block of code. If the condition is not
// met, do not run this block of code.”

// fn main() {
//     let num = 10;
//     if num > 100 {
//         println!("The number is greater than 100");
//     } else if num == 100{
//         println!("The number is equal to 100");
//     } else {
//         println!("Then number is less than 100");
//     }
 
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main () {
//     let condition = true;
//     let x = if condition {5} else {10};
//     println!("{}",x) 
// }

// fn main () {
//     let condition = true;
//     let x = if condition {5} else {"ten"};
//     println!("{}",x) 
// }

