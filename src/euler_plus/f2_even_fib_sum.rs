use std::io::{self, BufRead};

pub fn f2() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let mut m: Vec<u64> = vec![1,2];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        let mut sum: u64 = 0;
        let mut i:usize = 0;
        loop{
            if m.len() <= i{
                m.push(m[i-1]+m[i-2]);
            }
            if n < m[i] {
                break
            }
            if m[i] % 2 == 0 {
                sum += m[i];
            }
            i+=1;
        }
        println!("{}",sum)
    }
}
