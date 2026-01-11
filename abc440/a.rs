use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        x: usize,
        y: usize,
        //s: Chars, // Vec<char>
    }

    let mut ans = x;

    for i in 0..y {
        ans *= 2;
    }
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", ans);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
