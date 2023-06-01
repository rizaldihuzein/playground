use std::io::{self, BufRead};

fn f(n: i32, cases: &[Vec<i32>], width:&Vec<i32>) -> Vec<i32> {
    let mut vec:Vec<i32> = vec![0_i32;n as usize];
    let mut res;
    for i in 0..n{
        res = 100000_i32;
        for k in cases[i as usize][0]..=cases[i as usize][1]{
            if res > width[k as usize]{
                res = width[k as usize];
            }
        }
        vec[i as usize] = res;
    }
    vec
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let width: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let mut cases: Vec<Vec<i32>> = Vec::with_capacity(t as usize);

    for i in 0..t as usize {
        cases.push(Vec::with_capacity(2_usize));

        cases[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = f(t, &cases, &width);

    for i in 0..result.len() {
        println!("{}", result[i]);
    }
}

pub fn service_lane(){
    main();
}
