use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        q: usize,
        mut w: [Chars; q],
    }

    let ans = 0;

    for word in w {
        let mut takaFlag = true;
        let mut aoFlag = true;
        for e in word {
            // a s a h i
            if !s.contains(&e) {
                takaFlag = false;
            }
            if !t.contains(&e) {
                aoFlag = false;
            }
        }
        if takaFlag && aoFlag {
            println!("Unknown");
        } else if takaFlag {
            println!("Takahashi");
        } else if aoFlag {
            println!("Aoki");
        } else {
            println!("Unknown");
        }
    }
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
