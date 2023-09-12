// Panic: give error
// Immutable
// fn main() {
//     let x = 5;
//     println!("Value of x is {x}");
//     x = 6;
//     println!("New Value {x}");
// }

// mutable
// fn main(){
//     let mut x = 5;
//     println!("Value of x is {x}");
//     x = 6;
//     println!("New Value {x}");
// }

// Constannt
// fn main(){
//     const ONE_MINUTE: u8 = 60;
//     println!("Seconds in one minute is {ONE_MINUTE}");
// }

// Shadowing
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x in the outer scope is {x}");
}
