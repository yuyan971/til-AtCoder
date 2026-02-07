use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        s: Chars, // Vec<char>
    }

    let tmp = s[0];
    for e in s {
        if e != tmp {
            println!("No");
            return;
        }
    }
    println!("Yes");

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
