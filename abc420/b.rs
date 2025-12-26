use proconio::input;
use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n], // n人
    }

    let mut point_list = vec![0; n];

    for i in 0..m {
        //println!("{:?}", point_list);

        let count_0 = s.iter().filter(|x| x[i] == '0').count(); // C#のLinqっぽい
        let count_1 = n - count_0;
        let tag = if count_0 < count_1 { '0' } else { '1' };
        for j in 0..n {
            if s[j][i] == tag {
                point_list[j] += 1;
            }
        }
    }

    let &max = point_list.iter().max().unwrap();

    //println!("{:?}", point_list);

    for (idx, &value) in point_list.iter().enumerate() {
        if value == max {
            println!("{}", idx + 1);
        }
    }
}
