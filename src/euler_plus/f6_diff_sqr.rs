use std::io::{self, BufRead};

pub fn f6() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut v_sum:Vec<u64> = vec![1,3,6];
    let mut v_sum_sq:Vec<u64> = vec![1,5,14];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        for i in v_sum.len()..n as usize{
            v_sum.push(v_sum[i-1]+i as u64+1);
            v_sum_sq.push(v_sum_sq[i-1]+((i+1)*(i+1)) as u64);
        }
        println!("{}",(v_sum[(n-1)as usize]*v_sum[(n-1)as usize]) - v_sum_sq[(n-1)as usize])
    }
}
