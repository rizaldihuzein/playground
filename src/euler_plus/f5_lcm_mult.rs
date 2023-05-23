use std::io::{self, BufRead};

// lcm(a,b) = a*b / gcd(a,b)

fn gcd(mut a:i32, mut b:i32)->i32{
    let mut temp = a;
    if a<b{
        a = b;
        b = temp;
    }
    while b > 0{
        temp = a;
        a = b;
        b = temp%b;
    }
    a
}

// lcm calculation
fn lcm(a:i32, b:i32)->i32{
    a*b/gcd(a,b)
}

pub fn f5() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        let mut num:i32=1;
        for i in 2..=n{
            num = lcm(num,i);
        }
        println!("{}", num)
    }
}
