use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut ans = 0;

    for i in 0..a + 1 {
        for j in 0..b + 1 {
            for k in 0..c + 1 {
                if (i * 500 + j * 100 + k * 50) == x {
                    //println!("{i} {j} {k}");
                    ans += 1;
                }
            }
        }
    }
    //let yes =
    //let str: String = s.iter().collect();
    println!("{}", ans);
    //println!("{}", if yes { "yes" } else { "No" });
}
