use std::io::{self, BufRead};

fn resolve(n: i32) -> i32 {
    let mut sum:i32 = 0;
    let mut iter = n;
    while iter > 0{
        if iter%10 == 0{
            iter/=10;
            continue
        }
        if n % (iter%10) == 0 {
            sum+=1;
        }
        iter/=10;
    }
    sum
}

pub fn find_digit() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = resolve(n);

        println!("{}", result);
    }
}
