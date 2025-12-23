use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        y: usize,
    }
    for i_10000 in 0..=n {
        for j_5000 in 0..=(n - i_10000) {
            if i_10000 * 10000 + j_5000 * 5000 + (n - i_10000 - j_5000) * 1000 == y {
                println!("{} {} {}", i_10000, j_5000, n - i_10000 - j_5000);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);

    //let yes =
    //let str: String = s.iter().collect();
    //println!("{}", if yes { "yes" } else { "No" });
}
