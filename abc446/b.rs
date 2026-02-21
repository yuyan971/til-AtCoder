use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut use_list = HashSet::new();

    for i in 0..n {
        // 客の数だけ回す
        input! {
            l: usize
        }

        let mut isFinish = false;
        for p in 0..l {
            input! {
                k: usize
            }
            if isFinish {
                continue;
            }
            if use_list.insert(k) {
                println!("{}", k);
                isFinish = true;
                continue;
            }

            if p == l - 1 {
                println!("{}", 0);
            }
        }
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
