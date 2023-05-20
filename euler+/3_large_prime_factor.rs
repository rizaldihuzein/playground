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

fn largest_prime_factor(n:&mut u64)->u64{
    let mut l_prime:u64 = 0;
    let mut div:u64 = 2;
    while div*div <= *n{
        if *n%div == 0{
            if is_prime(div){
                l_prime = div;
            }
            if is_prime(*n/div) && *n/div > l_prime{
                l_prime = *n/div;
            }
        }
        div += 1;
    }
    if l_prime == 0{
        return *n;
    }
    
    l_prime  
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}", largest_prime_factor(&mut n))
    }
}
