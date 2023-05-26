use std::io::{self, BufRead};

// combinatory
// C(n,k) = n!/(k!*(n-k)!)
// where n = a+b
// wrong answer due to multiplication limitation?

const MODUL:isize = 1000000007;

fn fact(n:isize)->isize{
    if n == 0{
        return 1;
    }
    if n == 1{
        return 1;
    }
    let next = fact(n-1); //% MODUL;
    (n * next) //%MODUL;
}

fn f(n:isize, m:isize) -> isize{
    let n_fact = fact(n+m); //%MODUL;
    let a_fact = fact(n); //%MODUL;
    let b_fact = fact(m); //%MODUL;
    let divab = (a_fact*b_fact); //%MODUL;
    (n_fact/divab)%MODUL
}

pub fn f15_2() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<isize>().unwrap();

        let k = first_multiple_input[1].trim().parse::<isize>().unwrap();

        println!("{}",f(n, k))
    }
}
