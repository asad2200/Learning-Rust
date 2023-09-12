// Basic if else
// fn main() {
//     const CONDITION_NUMBER: u8 = 5;
//     // let number = 3;
//     let number = 7;

//     if number > CONDITION_NUMBER {
//         println!("Number is greater than {CONDITION_NUMBER}.");
//     } else {
//         println!("Number is lower than {CONDITION_NUMBER}.");
//     }
// }

// Rust do not convert number into boolean unlike other languages
// fn main() {
//     // let number = 3;
//     let number = 7;

//     // this block will give error
//     // if number {
//     //     println!("Number is available");
//     // }

//     if number != 0 {
//         println!("Number is available");
//     }
// }

// Example of if - else if - else
// fn main() {
//     let number = 5;
//
//     if number % 2 == 0 {
//         println!("Number is divided by 2.");
//     } else if number % 3 == 0 {
//         println!("Number is divided by 3.");
//     } else if number % 5 == 0 {
//         println!("Number is divided by 5.");
//     } else {
//         println!("Number is not divided by 2, 3, 5.");
//     }
// }

// Example of if in let statement
// fn main() {
//     let condition = true;
//     let a = if condition { 4 } else { 5 };

//     // let a = if condition { 4 } else { "five" }; this gives error at compile time

//     println!("The value of a id {a}.");
// }
