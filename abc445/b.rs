use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n], // Vec<char>
    }

    let mut max = 0;
    for e in &s {
        if max < e.len() {
            max = e.len()
        }
    }

    for e in s {
        let mut index = 0;
        let e_len = e.len();
        let num = (max - e.len()) / 2; // (5-3)/2 = 1
        for i in 0..max {
            if index < num || (max - 1) - num < index {
                print!(".");
            } else {
                print!("{}", e[index - num]);
            }
            index += 1;
        }
        println!("");
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
