use std::{io::{self, BufRead}};

fn f(n: i32, vec_path:&mut Vec<Vec<i32>>) -> i32{
    let mut dp_vec:Vec<Vec<i32>> = vec![];
    dp_vec.push(vec![vec_path[0][0]]);
    for i in 1..n as usize{
        dp_vec.push(Vec::with_capacity((i+1) as usize));
        for j in 0..=i as usize{
            let mut temp = 0;
            if j>0 && dp_vec[i-1][j-1]+vec_path[i][j] > temp{
                temp = dp_vec[i-1][j-1]+vec_path[i][j];
            }
            if j<i && dp_vec[i-1][j]+vec_path[i][j] > temp{
                temp = dp_vec[i-1][j]+vec_path[i][j];
            }
            if i+1 == n as usize && j > 0 && dp_vec[i][j-1] > temp{
                temp = dp_vec[i][j-1];
            } 
            dp_vec[i].push(temp);
        }
    }
    dp_vec[n as usize-1][n as usize-1]
}

pub fn f18() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let mut vec_path:Vec<Vec<i32>> = vec![];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        for i in 0 .. n{
            vec_path.push(Vec::with_capacity((i+1) as usize));

            let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
            
            for j in 0..(i+1) as usize{
                vec_path[i as usize].push(first_multiple_input[j].trim().parse::<i32>().unwrap());
            }
        }
        println!("{}",f(n, &mut vec_path));
        vec_path.clear();
    }
}
