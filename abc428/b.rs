use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars, // Vec<char>
    }

    let mut map = HashMap::new();

    for e in s.windows(k) {
        //println!("{:?}", e); // 最高かもしれん．
        *map.entry(e).or_insert(0) += 1;
    }
    //println!("{:?}", map);
    let max = map.values().max().unwrap(); // イテレータって結局何なんだろう...
    println!("{} ", &max);

    let mut words: Vec<String> = Vec::new();
    for (key, value) in &map {
        if value == max {
            words.push(key.iter().collect());
        }
    }
    words.sort();

    for i in 0..words.len() {
        print!("{}", words[i]);
        if i != words.len() - 1 {
            print!(" ");
        } else {
            println!();
        }
    }
}
