use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut ans = 0;
    let mut escape = false;
    loop {
        for e in &mut a {
            if *e % 2 != 0 {
                escape = true;
                break;
            }
            *e /= 2;
        }
        if escape {
            break;
        }
        ans += 1
    }
    //let yes =
    //let str: String = s.iter().collect();
    println!("{}", ans);
    //println!("{}", if yes { "yes" } else { "No" });
}
