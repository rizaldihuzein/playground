use std::io::{self, BufRead};

fn calc_greatest(n:i32,k:i32,num:&String) -> u64{
    let mut max_score:u64 = 0;
    let mut score:u64 = 1;
    let char_arr = num.as_bytes();
    let mut zero_flag = 0;
    for i in 0..(n as usize){
        let digit_now = char_arr[i] as u64 - '0' as u64;
        if digit_now == 0{
            zero_flag += 1;
        } else {
            score *= digit_now;
        }
        if i >= (k as usize) - 1{
            if i >= (k as usize){
                let digit_was = char_arr[i - (k as usize)] as u64 - '0' as u64;
                if digit_was == 0{
                    zero_flag -= 1;
                }else{
                    score /= digit_was;
                }   
            }
            if score > max_score && zero_flag == 0{
                max_score = score;
            }
        }
    }
    
    max_score
}

pub fn f8() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let num = stdin_iterator.next().unwrap().unwrap();
        
        println!("{}",calc_greatest(n,k,&num))
    }
}
