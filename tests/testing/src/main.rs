#![allow(dead_code, unused, clippy::unused_unit)]

use vary::{varargs, vary};

#[varargs]
pub async unsafe fn control<T>(a: String, b: T, c: &[u32]) -> T {
    unimplemented!()
}

fn generic<T, const ARITY: usize>(c: [T; ARITY]) {

}

#[varargs]

// #[vary]
// fn basic_homo(vec: Vec<u32>) {
//     (1, 2, 3)...;
//     ()
// }

fn main() {
    // basic_homo((1, 2, 3));
    println!("This compiles!");
}
