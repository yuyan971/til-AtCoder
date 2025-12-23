use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        s: Chars, // Vec<char>
    }
    let ans = a + b + c;
    let str: String = s.iter().collect();
    //let yes =

    println!("{} {}", ans, str);
    //println!("{}", if yes { "yes" } else { "No" });
}
