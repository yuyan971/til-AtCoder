use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        d: usize,
        f: usize,
    }

    let ans = 7 - ((d - f) % 7);
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", ans);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
