use std::io::{self, BufRead};
use std::collections::HashMap;

// 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

fn calc_step(n: usize, map:&mut HashMap<usize,u64>) -> u64{
    if n == 0{
        return 0;
    }
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    if map.contains_key(&n){
        return *map.get(&n).unwrap();
    }

    let next:u64;
    let n = n as u64;
    if n%2 == 0{
        next = n/2;
    }else{
        next = (3*n)+1;
    }
    let count = calc_step(next as usize, map) + 1;
    map.insert(n as usize, count);

    count
}

fn f(n:u64, map:&mut HashMap<usize,u64>, nums:&mut Vec<u64>) -> u64{
    let mut count ;
    let mut step: u64;
    for i in nums.len()..n as usize{
        count = *map.get(&(nums[i-1] as usize)).unwrap();
        step = calc_step(i+1,map);
        if count > step{
            nums.push(nums[i-1]);
        }else{
            nums.push(i as u64 +1);
        }
    }

    nums[n as usize -1]
}

pub fn f14() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let mut numbers: Vec<u64> = vec![1,2];
    let mut count_map:HashMap<usize,u64> = HashMap::new();
    count_map.insert(0,0);
    count_map.insert(1,0);
    count_map.insert(2,1);
    
    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}",f(n,&mut count_map,&mut numbers))
    }
}
