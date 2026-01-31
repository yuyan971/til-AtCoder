use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        s: Chars, // Vec<char>
    }

    //let mut yes =
    let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}s", str);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
