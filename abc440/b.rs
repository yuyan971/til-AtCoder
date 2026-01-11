use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut t: [usize; n],
    }

    let mut vec_new = Vec::new();
    for i in 0..n {
        vec_new.push((t[i], i + 1));
    }

    vec_new.sort();

    let mut count = 0;
    for e in vec_new {
        print!("{} ", e.1);
        count += 1;
        if count > 2 {
            break;
        }
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
