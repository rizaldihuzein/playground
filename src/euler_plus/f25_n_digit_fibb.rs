use std::{io::{self, BufRead}, vec};
use crate::general::{fibonacci::fib_digit,search::binary_search_left_most};

fn f(n_min:u64, vec_fib_digit:&mut Vec<u64>) -> u64{
    if vec_fib_digit.len() == 0{
        vec_fib_digit.push(0);
    }
    let mut index = vec_fib_digit.len()-1;
    if index > 0 && vec_fib_digit[index] >= n_min{
        index = binary_search_left_most(vec_fib_digit, n_min);
    }else{
        while vec_fib_digit[index] < n_min {
            index+=1;
            vec_fib_digit.push(fib_digit(index as u64));
        }   
    }
    
    index as u64
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut vec_fib_digit:Vec<u64> = vec![0,1,1];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}", f(n, &mut vec_fib_digit));
    }
}

pub fn f25() {
    main();
}
