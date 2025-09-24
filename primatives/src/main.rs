// Primitive data types in Rust
// int, float, bool, char

// Integer types
// Rust has signed and unsighted integers of various sizes:
// i8, i16, i32, i64, i128, (Signed integers)
// u8, u16, u32, u64, u128 (Unsigned integers) The different number corresponds to the number of bits the integer uses.
fn main(){
    let x: i32 = -5;
    let y: u64 = 10;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
// ========== Floating-point types ==========
    let a: f32 = 3.14; // 32-bit floating point
    let b: f64 = 2.718281828459045; // 64-bit floating point
    println!("32-bit Float: {}", a);
    println!("64-bit Float: {}", b);
// ========== Boolean type ==========
    let is_rust_fun: bool = true;
    let is_sky_green: bool = false;
    println!("Is Rust fun? {}", is_rust_fun);
    println!("Is the sky green? {}", is_sky_green);
// ========== Character type ==========
    let letter: char = 'R';
    let emoji: char = '😞';
    println!("what does the word 'road' start with? {}", letter);
    println!("how we feelin? {}", emoji);
}
