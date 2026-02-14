use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        s: Chars, // Vec<char>
    }

    if s[0] == s[s.len() - 1] {
        println!("Yes");
    } else {
        println!("No");
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
