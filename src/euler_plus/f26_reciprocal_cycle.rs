use std::{io::{self, BufRead}, vec};
use std::collections::HashMap;
use crate::ternary;

//https://en.wikipedia.org/wiki/Repeating_decimal#Converting_repeating_decimals_to_fractions
fn find_decimal_recurring_ndigit(n:u64) -> u64{
    let mut digit = 0_u64;
    let mut modulo:HashMap<usize,bool> = HashMap::new();
    let mut modul = 10%n;
    while modul > 0{
        if match modulo.get(&(modul as usize)){
            Some(v) => *v,
            None => false,
        }{
            break
        }
        modulo.insert(modul as usize, true);
        digit+=1;
        modul = (modul*10)%n;
    }
    ternary!(modul!=0,digit,0)
}

fn f(n:u64, vec_indice:&mut Vec<u64>, highest:&mut u64) -> u64{
    if vec_indice.len() == 0{
        *vec_indice = vec![0,0,0,3];
    }
    let mut index = vec_indice.len()-1;
    let mut to_push;
    while (index as u64) < n-1 {
        index+=1;
        let current = find_decimal_recurring_ndigit(index as u64);
        to_push = vec_indice[index -1];
        if current > *highest{
            *highest = current;
            to_push = index as u64;
        } 
        vec_indice.push(to_push);
    }
    
    vec_indice[n as usize -1]   
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut vec_indice:Vec<u64> = vec![0,0,0,3];
    let mut highest:u64 = 1;


    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}", f(n, &mut vec_indice, &mut highest));
    }
}

pub fn f26() {
    main();
}
