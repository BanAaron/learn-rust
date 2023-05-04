use std::io;

fn main() {
    // intro text
    println!("Welcome to the Rust quick reminder cheatsheet");
    println!("Press `Enter` to proceed in each step");

    // capture user input to pause between steps
    capture_user_input();
    scalars();
    capture_user_input();
    compounds();
    capture_user_input();
}

fn capture_user_input() {
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error capturing string");
}

fn scalars() {
    // scalars represents single values
    println!("Rust has the following scalar values.");

    // integers
    let integer_i32: i32 = -5; // can be signed (can be negative)
    let integer_u32: u32 = 5; // or unsigned (cannot be negative)
    println!("Integers! Signed or unsigned");
    println!("- integer_i32 = {integer_i32}");
    println!("- integer_u32 = {integer_u32}");

    // floating point types
    let float_f64: f64 = 5.55;
    let float_f32: f32 = 5.5;
    println!("Floats! 64 or 32 bit");
    println!("- float_f64 = {float_f64}");
    println!("- float_f32 = {float_f32}");

    // boolean 👻
    let bool_true: bool = true;
    let bool_false: bool = false;
    println!("Boolens!");
    println!("- boolean true = {bool_true}");
    println!("- boolean false = {bool_false:}");

    // char
    let a: char = 'a'; // must be declared with single quotes
    let jing: char = '静';
    let cat: char = '😺';
    println!("Chars!");
    println!("- a = {a}");
    println!("- jing = {jing}");
    println!("- cat = {cat}");
}

fn compounds() {
    // compound types can group multiple values into one type
    println!("Rust has the following compound types.");

    // tuple
    let tup: (char, f64, bool) = ('1', 1.5, true);
    println!("Tuples! They can contain a mix of all types and are of a fixed length");
    let (x, y, z) = tup;
    println!("- tuple(char, f64, bool) = ({x}, {y}, {z})");

    // lists
    // unlike other languages lists have a fixed length
    let _string_list: [&str; 3] = ["Aaron", "Chris", "Drew"];
    let _i32_list: [i32; 3] = [1, 2, 3];
    let _bool_list: [bool; 3] = [true, true, false];
    println!("Lists! They can only contain values of the same type.");
    println!("- string list = [\"Aaron\", \"Chris\", \"Drew\"]");
    println!("- i32 list = [1, 2, 3]");
    println!("- bool list = [true, true, false]");
}
