use std::{io::{self, BufRead}};

/*
    https://en.wikipedia.org/wiki/Zeller%27s_congruence#Implementations_in_software
    zeller congruence
    h = (q + [(13*[m+1])/5] + k + [k/4] + [J/4] + 5*J) % 7
    q = day of month 
    m = month 3 = may .. 14 = feb
    if m < 3, m+=12 y-=1 due to jan feb is counted as 13 14 last year
    k = year % 100
    J = year / 100
*/

fn check_date(mut y:u64,mut m:u8,d:u8) -> u8{
    if m < 3{
        m += 12;
        y-=1;
    }
    let m = m as u64;
    let d = d as u64;
    let k = y%100;
    let j = y/100;
    
    ((d + ((13*(m+1))/5) + k + (k/4) + (j/4) + (5*j)) % 7) as u8
}

fn f(dates:&[[u64;3];2]) -> u64{
    let mut count = 0;
    let mut m = dates[0][1];
    let mut d = dates[0][0];
    let mut lim_m = 12_u64;
    for i in dates[0][2]..=dates[1][2]{
        if i == dates[1][2]{
            lim_m = dates[1][1];
        }
        while m <= lim_m {
            
            if d > 1{
                d = 1;
                m+=1;
                continue;
            }
            if check_date(i, m as u8, d as u8) == 1{
                count+=1;
            }
            m+=1;
        }
        m=1;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let mut in_arr:[[u64;3];2] = [[0;3];2];

    for _ in 0..t {    
        for i in 0..2 as usize{
            let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
            
            for j in 0..3 as usize{
                in_arr[i][j] = first_multiple_input[3 - j - 1].trim().parse::<u64>().unwrap();
            }
            if in_arr[i][1] == 2 && in_arr[i][0] > 28{
                if in_arr[i][0] > 29 && (in_arr[i][2] % 4 == 0 && in_arr[i][2] % 100 != 0 || in_arr[i][2] % 400 == 0){
                    in_arr[i][0] -= 29;
                }else{
                    in_arr[i][0] -= 28;
                }
                if in_arr[i][0] < 28{
                    in_arr[i][1]+=1;
                }
            }
            
        }
        println!("{}",f(&in_arr));
        in_arr = [[0;3];2];
    }
}

pub fn f19(){
    main()
}
