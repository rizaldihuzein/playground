use std::{io::{self, BufRead}, vec, str};
use crate::general::gen_factorial::gen_factorial;

// https://oeis.org/wiki/Factorial_numeral_system
// index perm 21 of abcd
// 3*3! + 1*2! + 1*1! + 0

fn f(mut n:u64, vec_alph:&mut Vec<u8>, vec_fact:&mut Vec<u64>)->String{
    let mut res:Vec<u8> = Vec::new();
    let mut index;
    for i in (0..13).rev(){
        index = n / vec_fact[i];
        n -= vec_fact[i] * index;
        res.push(vec_alph[index as usize] + 'a' as u8);
        vec_alph.remove(index as usize);
    }
    match str::from_utf8(&res) {
        Ok(v) => v.to_string(),
        Err(_e) => "".to_string(),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut vec_alph:Vec<u8> = (0..13_u8).collect();
    let mut vec_fact = vec![];
    gen_factorial(12, &mut vec_fact);

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        let n = n-1;
        println!("{}", f(n, &mut vec_alph, &mut vec_fact));
        vec_alph = (0..13_u8).collect();
    }
}

pub fn f24() {
    main();
}
