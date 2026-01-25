use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut sum = 0;
    for e in a.iter().enumerate() {
        if e.0 == k {
            break;
        }
        sum += e.1;
    }
    if sum < x {
        println!("{}", -1);
        return;
    }

    a.reverse();

    let mut sum = 0;
    let mut sum_mayWater = 0;
    for e in a.iter().enumerate() {
        sum += e.1;
        if e.0 < (n - k) {
            sum_mayWater += e.1
        }

        if (sum - sum_mayWater) >= x {
            println!("{}", e.0 + 1);
            break;
        }
    }
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
