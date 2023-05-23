use std::io::{self, BufRead};
use std::str;

fn f(nums:&Vec<String>) -> String{
    let mut result:Vec<u8> = Vec::new();
    let mut byte_data: &[u8];
    let mut i_len:usize;
    let mut current:u8;
    let mut carry:u8;
    let limit_char = '0' as u8;
    let mut k = 0_usize;
    for i in nums{
        byte_data = i.as_bytes();
        i_len = byte_data.len();
        if result.len() > i_len{
            i_len = result.len();
        }
        carry = 0;
        current = 0;
        while k < i_len{
            if result.len()<k+1{
                result.push(limit_char);
            }
            current = result[k] - limit_char + carry;
            if k < byte_data.len(){
                current = current + byte_data[byte_data.len()-k-1] - limit_char;
            }
            carry = current/10;
            current = current%10;
            result[k] = current + limit_char;
            k+=1;
        }
        while carry != 0{
            result.push(carry % 10 + limit_char);
            carry = carry/10;
        }
        k=0;
    }
    result.reverse();
    let result_str = match str::from_utf8(&result) {
        Ok(v) => v,
        Err(e) => "",
    };
    result_str[0..10].to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let mut numbers: Vec<String> = Vec::with_capacity(t as usize);
    
    for i in 0..t {
        numbers.push(String::new());
        numbers[i as usize] = stdin_iterator.next().unwrap().unwrap().trim().to_string();
    }
    println!("{}",f(&numbers))
}
