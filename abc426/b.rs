use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars, // Vec<char>
    }

    let mut map = HashMap::new();
    for &e in &s {
        *map.entry(e).or_insert(0) += 1;
    }

    for e in &s {
        if map[e] == 1 {
            println!("{e}");
            return;
        }
    }

    /*
    let mut set = HashSet::new();

    for &e in &s {
        if !set.insert(e) {
            println!("{}", e);
            return;
        }
    }
    */
}
