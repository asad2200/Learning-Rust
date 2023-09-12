// 1. Scaler Types
// Integer, Float, booleans, characters

// Integer
// 8-bit	i8	   u8
// 16-bit	i16	   u16
// 32-bit	i32	   u32
// 64-bit	i64	   u64
// 128-bit	i128   u128
// arch	    isize  usize
// fn main() {
//     println!("Hello, world!");
// }

// f32 and f64
// fn main(){
//     let x = 2.0;

//     let y: f32 = 3.0;
// }

// Boolean Type
// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

// Character Type
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// 2. Compound Types
// tuple, list

// tup
// fn main(){
//     let tup: (i32, f64, u8) = (800, 6.4, 1);
//     let (x, y, z) = tup;

//     println!("The value of y is {y}");
//     println!("The value of second element in tuple is {}", tup.1);
// }

// tup
fn main(){
    // let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5]; // same value(3) for 5 times

    println!("b : {:?}", b);

    let first = a[0];
    let second = a[1];

    println!("First: {first}, Second: {second}");
}