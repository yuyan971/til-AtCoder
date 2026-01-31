use proconio::input;
//use proconio::marker::Chars;
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut a: [usize; n],
    }

    let mut sum = 0;
    a.push(0);

    let mut pointer = 0; // indexを指す
    let mut isOpening = true;
    let mut xTime = 0;
    for i in 0..t {
        if i != 0 && a[pointer] == i {
            //println!("{}", i);
            pointer += 1;
            if isOpening {
                isOpening = false;
                xTime = i + 100;
            }
        }

        if (isOpening == false) && (xTime == i) {
            isOpening = true;
        }

        if isOpening {
            sum += 1;
        }
    }

    //let mut yes =
    //let str: String = s.iter().collect(); // Vec<Char> To String
    println!("{}", sum);
    //println!("{}", if yes { "Yes" } else { "No" });
    //println!("Yes");
    //println!("No");
}
