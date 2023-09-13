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

// Loops ------------

// loop keyword
// fn main(){
//     loop{
//         println!("This is the infinite loop!");
//     }
// }

// returning value from loop
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}.");
// }

// loop with label
// fn main() {
//     let mut counter = 0;
//     'counting_up: loop {
//         println!("Counter : {counter}");
//         let mut remaining = 10;

//         loop {
//             println!("Remaining: {remaining}");

//             if remaining == 9 {
//                 break;
//             }

//             if counter == 2 {
//                 break 'counting_up;
//             }

//             remaining -= 1;
//         }

//         counter += 1;
//     }

//     println!("End counter = {counter}.");
// }

// while loop
// fn main() {
//     let mut count = 4;

//     while count != 0 {
//         println!("Count = {count}");

//         count -= 1;
//     }

//     println!("End of the Loop..");
// }

// for loop
// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//
//     for element in a {
//         println!("Value = {element}.");
//     }
// }

// Range with for loop
fn main() {
    // for number in 1..4 {
    for number in (1..4).rev() {
        println!("Number: {number}!");
    }

    println!("End of the loop....");
}
