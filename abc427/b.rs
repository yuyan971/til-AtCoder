use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
    }

    let a0 = 1;
    let mut sum = a0;
    for i in 0..n - 1 {
        sum += f2(sum);
    }
    let ans = sum;
    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", ans);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}

fn f(sum: usize) -> usize {
    // メモリ効率はよくないらしいので，下に代替．
    let str = sum.to_string();
    let ans = str.chars().map(|e| e.to_digit(10).unwrap() as usize).sum();
    return ans;
}

fn f2(mut sum: usize) -> usize {
    let mut ans = 0;
    while sum > 0 {
        ans += sum % 10;
        sum /= 10;
    }
    return ans;
}
