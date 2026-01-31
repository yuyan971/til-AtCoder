use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        t: usize,
    }

    for i in 0..t {
        input! {
            n: usize,
            mut r: [usize; n]
        }

        let mut step = 0;
        for k in 1..(n - 1) {
            for i in 0..n {
                if r[i] == k {
                    //println!("{} {}", i, r[i]);

                    if i != (n - 1) && r[i + 1] > (k + 1) {
                        step += r[i + 1] - (k + 1);
                        r[i + 1] = k + 1;
                    }
                    if i != 0 && r[i - 1] > (k + 1) {
                        step += r[i - 1] - (k + 1);
                        r[i - 1] = k + 1;
                    }
                }
            }
        }
        println!("{}", step);
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
