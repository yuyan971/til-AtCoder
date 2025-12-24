use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
use std::collections::BTreeSet;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n], // -1が来るので，isize
    }

    let mut unused: BTreeSet<usize> = (1..=n).collect();
    //println!("{:?}", unused);

    for (idx, &val) in a.iter().enumerate() {
        if val == -1 {
            continue;
        }
        if !unused.remove(&(val as usize)) {
            // ここ大事！ vecはインデックス指定しかできないが，BTreeSetはキー指定で消せる．
            println!("No");
            return;
        }
    }
    println!("Yes");
    //println!("{:?}", unused);

    for e in &mut a {
        if *e == -1 {
            // 値が未定ならば..
            *e = unused.pop_first().unwrap() as isize;
        }
    }
    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string()) // それぞれをstringに変換
            .collect::<Vec<String>>() // Vectorに変換
            .join(" ") // 1つの文字列に変換．
    );
}
