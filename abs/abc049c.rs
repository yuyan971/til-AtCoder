use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars, // Vec<char>
    }

    let words: Vec<Vec<char>> = vec![
        "dreamer".chars().collect(),
        "dream".chars().collect(),
        "eraser".chars().collect(),
        "erase".chars().collect(),
    ];

    loop {
        if s.is_empty() {
            println!("YES");
            return;
        }
        let mut flag = true;
        for e in &words {
            if s.ends_with(e) {
                // その文字列の長さの分だけsを削る
                s.truncate(s.len() - e.len());
                flag = false;
                break;
            }
        }
        if flag {
            println!("NO");
            return;
        }
    }
}
