use std::{io::{self, BufRead}, vec};
use crate::general::prime::gen_primesieve;

fn f(n:i64, sieve:&mut Vec<bool>) -> (i64,i64){
    let mut chosen_a = 0;
    let mut chosen_b = 0;
    let mut chosen_consec = 0;
    let mut current_consec;
    let mut count;

    for a in -n..=n{
        for b in -n..=n{
            current_consec = 0;
            count = current_consec*current_consec+a*current_consec+b;
            if count > 0 && count as usize >= sieve.len(){
                gen_primesieve(sieve, count as u64);
            }
            while current_consec < n && count > 0 && sieve[count as usize]{
                current_consec+=1;
                count = current_consec*current_consec+a*current_consec+b;
                if count > 0 && count as usize >= sieve.len(){
                    gen_primesieve(sieve, count as u64);
                }
            }
            if chosen_consec < current_consec{
                chosen_consec = current_consec;
                chosen_a = a;
                chosen_b = b;
            }
        }
    }

    (chosen_a, chosen_b)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut sieve:Vec<bool> = vec![];
    gen_primesieve(&mut sieve, 2000_u64);
    
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();
    let res = f(n, &mut sieve);
    println!("{} {}", res.0, res.1);
}

pub fn f27() {
    main();
}
