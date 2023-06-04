use std::io::{self, BufRead};
use crate::ternary;


/*
 * Complete the 'angryProfessor' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY a
 */

fn f(k: i32, a: &[i32]) -> String {
    let prof_no_angry = a.iter().filter(|&x| *x < 1).collect::<Vec<&i32>>();
    ternary!(prof_no_angry.len() >= k as usize , "NO".to_string() , "YES".to_string())
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = f(k, &a);

        println!("{}", result);
    }
}

pub fn angry_prof(){
    main();
}
