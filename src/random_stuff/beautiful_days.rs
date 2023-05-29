use std::io::{self, BufRead};

/*
 * Complete the 'beautifulDays' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER i
 *  2. INTEGER j
 *  3. INTEGER k
 */

fn flip_num(mut n:i32)->i32{
    let mut res = 0;
    while n > 0{
        res = (res*10) + (n%10);
        n/=10;
    }
    res
}

fn f(mut i: i32, mut j: i32, k: i32) -> i32 {
    let mut res = 0;
    let mut curr;
    if i > j{
        let temp = i;
        i = j;
        j = temp;
    }
    for n in i..=j{
        let rot = flip_num(n);
        curr = rot - n;
        if curr < 0{
            curr *= -1;
        }
        if curr%k==0{
            res+=1;
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let i = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let j = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let result = f(i, j, k);

    println!("{}", result);
}

pub fn beautiful_days(){
    main();
}
