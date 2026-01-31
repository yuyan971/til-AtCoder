use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }

    let mut sum = 0;
    let mut count = 0;
    loop {
        sum += n;

        if sum >= k {
            break;
        }

        count += 1;
        n += 1;
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", count);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
