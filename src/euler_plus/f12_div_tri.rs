use std::io::{self, BufRead};

// vec store sum
// vec store num factors

// 1,2,3,4,6,9,12,18,36
// 1,3,5,9,15,45

fn count_factor(a:u64)->u64{
    let mut i:u64 = 1;
    let mut counter = 0;
    while i*i<=a{
        if i*i == a{
            counter+=1;
        }else if a%i==0{
            counter+=2;
        }
        i+=1;
    }
    
    counter
}

fn f(sum:&mut Vec<u64>, num_fac:&mut Vec<u64>, n_min:u64) -> u64{
    let mut index = num_fac.len() - 1;
    if index > 0 && num_fac[index] > n_min{
        index = binary_search_right_most(num_fac, n_min);
    }else{
        let mut new_fac:u64;
        while num_fac[index] <= n_min {
            sum.push(index as u64+2+sum[index]);
            new_fac = count_factor(index as u64+2+sum[index]);
            if new_fac < num_fac[index]{
                new_fac = num_fac[index];
            }
            num_fac.push(new_fac);
            
            index+=1;
        }   
    }
    
    sum[index]
}

fn binary_search_right_most(slice:&Vec<u64>, n_min:u64)->usize{
    let mut start:usize = 0;
    let mut end = slice.len() - 1;
    let mut pos = (start+end)/2;
    if pos == 0{
        return pos;
    }
    loop{
        if slice[pos]<=n_min{
            start = pos;
        }else{
            end = pos;
        }
        pos = (start+end)/2;
        if end-start == 1{
            break;
        }
    }
    
    pos+1
}

pub fn f12() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut triangle_sum: Vec<u64> = vec![1,3,6,10,15,21,28];
    let mut vactors_track: Vec<u64> = vec![1,2,4,4,4,4,6];

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}",f(&mut triangle_sum,&mut vactors_track, n))
    }
}
