use std::io::{self, BufRead};

pub fn f1() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();
        n -=1; 
        let sum_3: i64 = ((n/3 * (n/3 + 1))/2)*3;
        let sum_5: i64 = ((n/5 * (n/5 + 1))/2)*5;
        let sum_15: i64 = ((n/15 * (n/15 + 1))/2)*15;
        println!("{}", sum_3+sum_5-sum_15);
    }
}
