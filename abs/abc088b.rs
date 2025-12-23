use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort(); // 2 3 7 9 10
    a.reverse(); // 10 9 7 3 2
    //println!("{:?}", a);
    let mut alice_sum = 0;
    let mut bob_sum = 0;
    for i in 0..n {
        if i % 2 == 0 {
            alice_sum += a[i];
        } else {
            bob_sum += a[i];
        }
    }
    let ans = alice_sum - bob_sum;
    //let yes =
    //let str: String = s.iter().collect();
    println!("{}", ans);
    //println!("{}", if yes { "yes" } else { "No" });
}
