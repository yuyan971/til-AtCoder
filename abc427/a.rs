use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars, // Vec<char>
    }

    s.remove((s.len() + 1) / 2 - 1);

    //let mut yes =
    let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", str);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
