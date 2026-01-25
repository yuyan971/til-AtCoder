use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut vol = 0;
    let mut isPlaying = false;
    for i in 0..n {
        input! {
            a: usize, // Vec< Vec<usize> >
        }

        if a == 1 {
            vol += 1;
        } else if a == 2 {
            if vol >= 1 {
                vol -= 1;
            }
        } else {
            // 3
            isPlaying = !isPlaying;
        }
        if vol >= 3 && isPlaying {
            println!("Yes");
        } else {
            println!("No");
        }
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
