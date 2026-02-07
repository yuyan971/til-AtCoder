use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut count = 0;
    for mut i in 1..=n {
        let mut sum = 0;
        loop {
            sum += i % 10;
            i /= 10;
            if i == 0 {
                break;
            }
        }

        if sum == k {
            count += 1;
        }
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", count);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
