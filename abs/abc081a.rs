use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars, // Vec<char>
    }

    let mut count = 0;
    for e in s {
        if e == '0' {
            continue;
        }
        count += 1;
    }
    //let yes =
    //let str: String = s.iter().collect();
    println!("{}", count);
    //println!("{}", if yes { "yes" } else { "No" });
}
