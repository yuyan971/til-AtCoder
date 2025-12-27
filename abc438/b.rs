use proconio::input;
use proconio::marker::Bytes;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Bytes, // Vec<char>
        t: Bytes, // Vec<char>
    }

    let mut min_step = 10000;
    for e in s.windows(m) {
        //println!("{:?}", e);
        let mut require_step = 0;
        for i in 0..m {
            let mut tmp = (e[i] as i32 - t[i] as i32) as isize;
            if tmp < 0 {
                tmp += 10;
            }
            require_step += tmp;
        }
        if require_step < min_step {
            min_step = require_step;
        }
    }
    println!("{min_step}");
}
