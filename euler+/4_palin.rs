use std::io::{self, BufRead};

fn palindrome_check(mut a:i32)->bool{
    if a%10 == 0{
        return false;
    }
    let mut half:i32 = 0;
    while a >= half{
        if a ^ half == 0{
            return true;
        }
        half*=10;
        half+=(a%10);
        a/=10;
        if half > 0 && a/half == 10{
            a/=10;
        }
    }
    false
}

fn is_largest_set(n:i32)->i32{
    let mut ans:i32 = 101101;
    let mut num:i32 = ans;
    let mut iter:i32 = 101;
    while num < n{
        if !palindrome_check(num){
            num+=1;
            continue;
        }
        iter = 101;
        while iter*iter <= num{
            if num%iter == 0 && num/iter/1000 == 0 && num/iter/100>0 && num > ans{
                ans = num;
                break;
            }
            iter+=1
        }
        num+=1;
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        println!("{}",is_largest_set(n))
    }
}
