use std::{io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut vec_names:Vec<String> = vec![];

    for _ in 0..t {
        let name = stdin_iterator.next().unwrap().unwrap().trim().to_string();
        vec_names.push(name.to_string());
    }
    vec_names.sort();

    t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let mut score: u64 = 0;
    for _ in 0..t{
        let name = stdin_iterator.next().unwrap().unwrap().trim().to_owned();
        let index =  match vec_names.iter().position(|x| *x == name){
            Some(i) => i+1,
            None => 0_usize,
        };
        if index == 0{
            println!("0");
            continue;
        }
        for a in vec_names[index-1].bytes(){
            score += a as u64 - 'A' as u64 + 1;
        }
        println!("{}", score * index as u64);
        score = 0;
    }
}

pub fn f22() {
    main();
}
