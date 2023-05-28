use std::{io::{self, BufRead}};

/*
    yuck >:p i hate this one,
    too many ifs and might need revisit
*/

static ARR1: [&str;16] = ["Zero", "One", "Two", "Three", "Four", "Five","Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen"];
static ARR2: [&str;4] = ["Twen", "Thir", "For", "Fif"];
static ARR3: [&str;4] = ["Thousand", "Million", "Billion", "Trillion"];

fn f(mut n: u64) -> String{
    let mut vec_str:Vec<String> = Vec::new();
    let mut i = 0;
    let mut modul:u64;
    let mut str_digit:String;
    let mut current:u64;

    while n>0{
        modul = n%1000;
        current = modul%10;

        //satuan
        str_digit = ARR1[current as usize].to_string();
        vec_str.push(str_digit.clone());
        let satuan = current;

        //puluhan
        modul /= 10;
        current = modul%10;
        if str_digit == ARR1[0].to_string(){
            vec_str.pop();
            str_digit = "".to_string();
        }
        if current > 5{
            str_digit = ARR1[current as usize].to_string();
            if current == 8{
                str_digit = "Eigh".to_string();
            }
            str_digit.push_str("ty");
            vec_str.push(str_digit.clone());
        }
        else if current > 1{
            str_digit = ARR2[current as usize - 2].to_string();
            str_digit.push_str("ty");
            vec_str.push(str_digit.clone());
        }
        else if current == 1{
            if satuan > 0{
                vec_str.pop();
            }
            if satuan > 5{
                str_digit = ARR1[satuan as usize].to_string();
                if satuan == 8{
                    str_digit = "Eigh".to_string();
                }
                str_digit.push_str("teen");
                vec_str.push(str_digit.clone());
            }
            else{
                str_digit = ARR1[satuan as usize+10].to_string();
                vec_str.push(str_digit.clone());
            }
        }

        //ratusan
        modul /= 10;
        current = modul%10;
        if str_digit ==  ARR1[0].to_string(){
            vec_str.pop();
        }
        if current > 0{
            vec_str.push("Hundred".to_string());
            vec_str.push(ARR1[current as usize].to_string());
        }

        n/=1000;
        if n%1000 > 0{
            vec_str.push(ARR3[i].to_string());
        }
        i+=1;
    }

    if vec_str.len() == 0{
        vec_str.push(ARR1[0].to_string())
    }

    vec_str.reverse();
    vec_str.join(" ")
}

pub fn f17() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("{}",f(n));
    }
}
