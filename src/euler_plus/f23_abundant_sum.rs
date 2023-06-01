use std::{io::{self, BufRead}};
use crate::general::divisor_sum::get_factor_sum;

fn f(n:u64, vec_sum:&mut Vec<u64>)->&str{
    if n > 28123{
        return "YES";
    }
    let mut dn;
    for i in 1..=(n/2) as usize{
        dn = vec_sum[i];
        if dn == 0{
            dn = get_factor_sum(i as u64);
            vec_sum[i] = dn;
        }
        if dn <= i as u64 {
            continue;
        }
        dn = vec_sum[n as usize-i];
        if dn == 0{
            dn = get_factor_sum(n-i as u64);
            vec_sum[n as usize-i] = dn;
        }
        if dn > (n-i as u64){
            return "YES";
        }
    }
    "NO"
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut vec_sum:Vec<u64> = vec![0;1000001];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}", f(n, &mut vec_sum));
    }
}

pub fn f23() {
    main();
}
