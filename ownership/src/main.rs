// understanding the variable scope and how value is cleared from heap
// fn main() {
//     {
//         let mut s = String::from("Hello"); // s allocated to memory by memory allocator

//         s.push_str(", World!");

//         println!("{s}");
//     }
//     // scope is over now and s will be free automatically by Rust
//     // rust call drop fucntion here
// }

// // Understanding how assigning value works
// fn main() {
//     let s1 = String::from("Test String");
//     let s2 = s1;

//     // this gives compile time error
//     // when we assign variable to another value first varaible will not be valid
//     // this is not true for fixed size data types like int, float, char, bool, tup(fixed_size data types)
//     // println!("{s1}");

//     println!("{s2}");

//     let x = 5;
//     let y = x;

//     println!("{x}");
//     println!("{y}");
// }

// Cloning in RUST
// fn main() {
//     let s1 = String::from("Test String");
//     let s2 = s1.clone();

//     // Unlike above example both s1 and s2 are valid, since we used clone()
//     println!("s1 = {s1}");
//     println!("s2 = {s2}");
// }

// Ownership with function
// Ownership works same as variable
// fn main(){
//     let s1 = String::from("Hello");

//     func(s1); // s1 is not valid from here, Ownership moved to function
//     // println!("{s1}"); Error:

//     let x = 5;
//     func_2(x); // Deep Copy

//     println!("{x}");
// }

// fn func(some_string: String){
//     println!("{some_string}");
// }

// fn func_2(some_int: u32){
//     println!("{some_int}");
// }

// return also work same as above
// fn main() {
//     let s1 = gives_ownership(); // function giving ownership to vaaraible

//     println!("{s1}");

//     let s2 = String::from("String Two");

//     let s3 = takes_and_gives_ownership(s2); // s2 moved to the function

//     println!("{s3}");
// }

// fn gives_ownership() -> String {
//     let s = String::from("Some String");

//     s
// }

// fn takes_and_gives_ownership(some_string: String) -> String {
//     some_string
// }

// function returns multiple values
fn main() {
    let s1 = String::from("Some String");

    let (s2, len) = find_len(s1);

    println!("The lenght of {s2} is {len}.");
}

fn find_len(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
