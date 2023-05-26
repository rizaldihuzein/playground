use std::io::{self, BufRead};

// combinatory
// C(n,k) = n!/(k!*(n-k)!)
// where n = a+b

const MODUL:i64 = 1000000007;

fn f(n: usize, m: usize, slice_table:&mut [[i64;501];501]) -> i64{
    for i in 0..n{
        slice_table[i][0] = i as i64+2;
    }
    for i in 0..m{
        slice_table[0][i] = i as i64+2;
    }
    for i in 1..n{
        for j in 1..m{
            slice_table[i][j] = (slice_table[i-1][j]+slice_table[i][j-1])%MODUL;
        }
    }
    (slice_table[n as usize-1][m as usize-1])%MODUL
}

pub fn f15() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut slice_table:[[i64;501];501] = [[0_i64;501];501];
    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i64>().unwrap();

        let k = first_multiple_input[1].trim().parse::<i64>().unwrap();

        println!("{}",f(n as usize, k as usize, &mut slice_table));
    }
}
