use std::io::{self, BufRead};

fn is_prime(a:u64)->bool{
    let mut i:u64 = 2;
    while i*i<=a{
        if a%i==0{
            return false;
        }
        i+=1;
    }
    return true;
}

fn resolve(vec:&mut Vec<u64>, pos:usize)->u64{
    let mut curr_sum:u64 = 0;
    for i in vec.len()+1..=pos as usize{
          curr_sum = vec[i-2];
          if is_prime(i as u64){
              curr_sum += i as u64;
          }
          vec.push(curr_sum);
    }
    
    vec[pos as usize - 1]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut prime_sum: Vec<u64> = vec![0,2,5,5,10];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        println!("{}",resolve(&mut prime_sum, n as usize))
    }
}
