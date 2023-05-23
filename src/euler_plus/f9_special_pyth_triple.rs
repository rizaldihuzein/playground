use std::io::{self, BufRead};

/*
    (a+b+c)^2 = N^2
    2(a^2 + b^2 + ac+bc+ab) = ...
    2(aN + b^2 + bc) = ...
    2(b^2 + b(N-a-b)) = N^2 - 2Na
    2Nb-2ba = N^2 - 2Na
    b = (N^2 - 2Na) / (2N-2a)
*/

fn get_mult_of_pith(n:i64)->i64{
    let mut res:i64 = -1;
    let mut b:i64;
    let mut c:i64;
    for a in 1..=n/3{
        b = (n*n - 2*n*a) / (2*n-2*a);
        c = n-a-b;
        if a*a+b*b==c*c && a*b*c > res{
            res = a*b*c;
        }
    }
    res
}

pub fn f9() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();
        println!("{}",get_mult_of_pith(n));
    }
}
