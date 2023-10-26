use crate::general::power::power;
use std::io::{self, BufRead};

fn f(n: u64) -> u64 {
    let mut ans: u64 = 0;
    let mut calc: u64;
    let mut sum: u64;
    for i in 10 as u64..1000000 as u64 {
        calc = i;
        sum = 0;
        while calc > 0 {
            sum += power((calc % 10) as u64, n as i64) as u64;
            calc /= 10;
        }
        if sum == i {
            ans += i;
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();
    let res = f(n);
    println!("{}", res);
}

pub fn f30() {
    main();
}
