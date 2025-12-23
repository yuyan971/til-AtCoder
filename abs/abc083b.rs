use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut ans = 0;

    for i in 1..n + 1 {
        // iの各桁の和の計算
        let sum = sum_digits(i);
        if (a..=b).contains(&sum) {
            ans += i;
        }
    }

    println!("{}", ans);
}

fn sum_digits(mut num: usize) -> usize {
    let mut sum = 0;
    loop {
        sum += num % 10;
        num /= 10;
        if num <= 0 {
            break;
        }
    }
    return sum;
}
