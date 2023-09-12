// fn main() {
//     println!("Hello, world!");

//     another_func()
// }

// fn another_func(){
//     println!("This is the second function");
// }

// fn main() {
//     println!("Hello, world!");

//     another_func(3);
// }

// fn another_func(a: i32) {
//     println!("This is the second function");
//     println!("The passed parameter a is {a}.");
// }

// Statement - Does not return the value in Rust
// Expression - returns the value ex. calling function, macro like println etc..
// fn main() {
//     // let a = b = 6; this is not possible in RUST
//     // let a = (let b = 6);

//     let a = {
//         let b = 6;
//         b
//     };

//     println!("The value of a is {a}.");
// }

// function returns the value
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = sum(2, 2);

    println!("Answer is {sum}.");
}
