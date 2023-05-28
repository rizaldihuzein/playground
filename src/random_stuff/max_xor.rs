use std::io::{self, BufRead};

// l ^ r will have constant 1st bit as its max
// 1100 ^ 0010 will have max 01xx
// 1000 ^ 0111 is the answer

fn resolve(l: i32, r: i32) -> i32 {
    let mut ans = 1;
    let mut xor = l^r;
    while xor > 0{
        ans<<=1;
        xor>>=1;
    }
    ans - 1
}

pub fn max_xor() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let l = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let r = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let result = resolve(l, r);
    println!("{}", result);
}
