// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

use std::iter;

fn main() {
    let a: [i32; 1001] = iter::repeat(10)
        .take(1001)
        .into_iter()
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
