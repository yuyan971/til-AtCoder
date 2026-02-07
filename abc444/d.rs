use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::collections::HashSet;

// fn main() { // サンプルケースは通っちゃったけど，ちゃんとTLEだった．
//     input! {
//         n:usize,
//         mut a: [usize; n]
//     }
//     let mut sum: u128 = 0;
//     for e in a {
//         let mut mul: u128 = 1;
//         for i in 1..=e {
//             sum += 1 * mul;
//             mul *= 10;
//         }
//     }

//     println!("{}", sum);
// }

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    a.sort();
    let mut b: VecDeque<usize> = VecDeque::new();
    for e in &a {
        b.push_back(*e);
    }

    let mut vec = Vec::new();
    for i in 0..(a[a.len() - 1]) {
        while !b.is_empty() && b[0] < (i + 1) {
            // 3 4
            b.pop_front();
        }
        vec.push(b.len());
    }
    for i in 0..vec.len() {
        if vec[i] > 9 {
            let num = vec[i] / 10;
            vec[i] = vec[i] % 10;
            if i != vec.len() - 1 {
                vec[i + 1] += num;
            } else {
                vec.push(num); // 本当はこの実装が間違ってることに気づいた．例 3を100回入力
                               // 途中でvecにpushしたところで，vecのループ回数が1増えたりはしてくれないからね．
            }
        }
        //println!("{}", i);
    }
    vec.reverse();
    for e in vec {
        print!("{}", e);
    }
}
