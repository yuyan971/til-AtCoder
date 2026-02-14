use proconio::input;
use rpds::set;
//use proconio::marker::Chars;
//use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n], // Vec< Vec<usize> >
    }

    let mut short: Vec<isize> = vec![-1; n];
    //let mut set_num: Vec<(HashSet<usize>, usize)> = Vec::new();

    for i in 0..n {
        let mut now_pos = i;

        // let mut ok = false;
        // for e in &set_num {
        //     if e.0.contains(&now_pos) {
        //         print!("{} ", e.1);
        //         ok = true;
        //         break;
        //     }
        // }

        // if ok {
        //     continue;
        // }

        let mut set = HashSet::new();
        set.insert(now_pos);
        for _ in 0..a.len() + 1 {
            if short[now_pos] != -1 {
                print!("{} ", short[now_pos]);
                break;
            }
            //println!("{}", now_pos);
            now_pos = a[now_pos] - 1;

            if !set.insert(now_pos) {
                print!("{} ", now_pos + 1);
                for e in set {
                    short[e] = now_pos as isize + 1;
                }

                //set_num.push((set, now_pos + 1));
                break;
            }
            //vec.push(now_pos + 1);
        }
        //println!("{:?}", set);
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
