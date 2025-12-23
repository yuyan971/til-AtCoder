use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    //let ans = 0;
    let yes = if (a * b) % 2 == 0 { true } else { false };
    //let str: String = s.iter().collect();
    //println!("{}", ans);
    println!("{}", if yes { "Even" } else { "Odd" });
}
