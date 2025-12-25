use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        mut prev1_prev2: (usize,usize), // 右の方が古い
    }

    for i in 0..8 {
        let new_num = calc(&prev1_prev2);
        //println!("{} {} {new_num}", prev1_prev2.1, prev1_prev2.0);
        prev1_prev2.0 = prev1_prev2.1;
        prev1_prev2.1 = new_num;
    }

    let ans = prev1_prev2.1;
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", ans);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}

fn calc(&prev1_prev2: &(usize, usize)) -> usize {
    let num = prev1_prev2.0 + prev1_prev2.1;
    let s = num.to_string();
    let rev_s: String = s.chars().rev().collect();
    return rev_s.parse::<usize>().unwrap();
}
