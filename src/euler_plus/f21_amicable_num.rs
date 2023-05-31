use std::io::{self, BufRead};

fn get_factor_sum(n:u64)->u64{
    let mut sum:u64 = 1;
    let mut i:u64 = 2;
    while i*i<=n{
        if n%i==0{
            sum+=i;
            if i*i!=n{
                sum+= n/i;
            }
        }
        i+=1;
    }
    sum
}

fn f(n:u64, vec_sum:&mut Vec<u64>)->u64{
    let mut sum=0_u64;
    let mut dn;
    for i in 1..=n as usize{
        if vec_sum[i] == 0{
            dn = get_factor_sum(i as u64);
            vec_sum[i] = dn;
        }
        if i as u64 == vec_sum[i]{
            continue;
        }
        dn = 0;
        let index = vec_sum[i] as usize;
        if index <= vec_sum.len(){
            dn = vec_sum[index];
        }
        if dn == 0{
            dn = get_factor_sum(vec_sum[i]);
        }
        if index <= vec_sum.len(){
            vec_sum[index] = dn;
        }
        if i as u64 == dn{
            sum+=i as u64
        }
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut vec_sum:Vec<u64> = vec![0;1000001];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}", f(n, &mut vec_sum));
    }
}

pub fn f21() {
    main();
}
