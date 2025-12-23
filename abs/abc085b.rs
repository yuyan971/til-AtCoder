use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }
    d.sort();
    d.reverse();

    let mut ans = 1; // 一番大きいものは必ず使う
    for i in 1..n {
        if d[i - 1] > d[i] {
            ans += 1;
        }
    }
    //let yes =
    //let str: String = s.iter().collect();
    println!("{}", ans);
    //println!("{}", if yes { "yes" } else { "No" });
}
