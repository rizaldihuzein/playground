use std::io::{self, BufRead};

fn mult_big_int(n:i32, res_vec:&mut Vec<u8>){
    if res_vec.len() == 0{
        res_vec.push(1);
    }
    let mut carry = 0;
    let mut current;
    for i in 0..res_vec.len(){
        current = (res_vec[i] as i32 * n) + carry;
        carry = current/10;
        res_vec[i] = (current % 10) as u8;
    }
    while carry > 0{
        res_vec.push((carry%10) as u8);
        carry /= 10;
    }
}

// limited to u64
// hopefully its not that huge :')
fn f_and_pop_and_print(n:i32, res_vec:&mut Vec<u8>){
    res_vec.push(1);
    for i in 2..=n{
        mult_big_int(i, res_vec);
    }
    
    let mut len_res = res_vec.len();
    let mut sum:u64 = 0;
    while len_res > 0{
        sum += match res_vec.pop(){
            Some(t) => t as u64,
            None => 0_u64
        };
        len_res -= 1;
    }
    println!("{}", sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut mult_fec:Vec<u8> = Vec::new();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        f_and_pop_and_print(n,&mut mult_fec);
    }
}

pub fn f20() {
    main();
}
