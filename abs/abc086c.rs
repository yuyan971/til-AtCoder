use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        txy: [(usize, isize, isize); n],
    }

    let mut prev_txy = (0, 0, 0);
    for txy in txy {
        let step = txy.0 - prev_txy.0;
        let min_len = ((txy.1 - prev_txy.1).abs() + (txy.2 - prev_txy.2).abs()) as usize;
        if min_len <= step && (step - min_len) % 2 == 0 {
            prev_txy = txy;
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
