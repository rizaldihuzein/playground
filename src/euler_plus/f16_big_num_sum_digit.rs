use std::io::{self, BufRead};

fn pow_2_reuse_vec(n:i32,res_vec:&mut Vec<u8>){
    res_vec.push(2);
    for _ in 1..n{
        let len_vec = res_vec.len();
        let mut carry=0;
        for p in 0..len_vec{
            let curr = res_vec[p]*2;
            res_vec[p] = (curr%10) + carry;
            carry = curr/10;
        }
        if carry > 0{
            res_vec.push(carry);
        }
    }
}

// limited to u64
// hopefully its not that huge :')
fn f_and_pop_and_print(res_vec:&mut Vec<u8>){
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

pub fn f16() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut pow_vec:Vec<u8> = Vec::new();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        pow_2_reuse_vec(n,&mut pow_vec);
        f_and_pop_and_print(&mut pow_vec);
    }
}
