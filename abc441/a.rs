use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }

    if (p <= x) && (p + 100 > x) && (q <= y) && (q + 100 > y) {
        println!("Yes");
    } else {
        println!("No");
    }

    //println!("Yes");
    //println!("No");
}
