use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 1;

    for i in 0..n {
        ans *= 2;
    }

    ans -= 2 * n;
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", ans);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
