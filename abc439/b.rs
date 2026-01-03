use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        mut n: usize,
    }

    let ans = 0;
    let mut used = HashSet::new();

    loop {
        let vec = divide_digitize(n);
        n = vec.iter().map(|x| x * x).sum();
        if !used.insert(n) {
            println!("No");
            return;
        }
        if n == 1 {
            println!("Yes");
            return;
        }
    }
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}

fn divide_digitize(n: usize) -> Vec<usize> {
    if n > 0 {
        let mut num = n;
        let mut result = Vec::new();
        while num != 0 {
            result.push((num % 10).try_into().unwrap());
            num /= 10;
        }
        result.reverse();
        result
    } else {
        vec![0]
    }
}
