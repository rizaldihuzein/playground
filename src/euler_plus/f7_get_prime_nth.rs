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

fn get_prime(vec:&mut Vec<u64>, pos:i32) -> u64{
    for i in vec.len()..pos as usize{
        let mut start = vec[i-1] + 1;
        loop{
            if is_prime(start){
                vec.push(start);
                break;
            }
            start+=1;
        }    
    }
    
    vec[pos as usize - 1]
}

pub fn f7() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut prime_vec: Vec<u64> = vec![2,3,5,7,11,13];
    
    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        println!("{}",get_prime(&mut prime_vec, n))
    }
}
