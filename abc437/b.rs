use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[usize; w]; h],
        b: [usize; n],
    }

    let mut maxCount = 0;
    let mut count1 = 0;
    for row in &a {
        count1 = 0;
        for num in row {
            for e in &b {
                if e == num {
                    count1+=1;
                    break;
                }
            }

        }
        if count1 > maxCount {
            maxCount = count1.clone();
        }
    }


    //let yes = 

    println!("{}", maxCount);
    //println!("{}", if yes { "yes" } else { "No" });
}
