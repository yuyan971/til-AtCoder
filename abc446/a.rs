use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        s: String, // Vec<char>
    }

    println!("Of{}", s.to_lowercase());
}
