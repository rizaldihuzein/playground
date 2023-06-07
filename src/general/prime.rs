pub fn is_prime(a:u64)->bool{
    let mut i:u64 = 2;
    while i*i<=a{
        if a%i==0{
            return false;
        }
        i+=1;
    }
    return true;
}

pub fn get_and_gen_nthprime(vec:&mut Vec<u64>, pos:i32) -> u64{
    for i in vec.len()..pos as usize{
        let mut start = vec[i-1] + 1;
        loop{
            if is_prime(start){
                vec.push(start);
                break;
            }
            start+=1;
        }    
    }
    
    vec[pos as usize - 1]
}

pub fn gen_primesieve(vec:&mut Vec<bool>, n:u64){
    *vec = vec![true;n as usize+1];
    vec[0] = false;
    vec[1] = false;
    let mut j:usize;
    for i in 2..=(n as usize+1)/2{
        j = i*2;
        while j <= n as usize{
            vec[j] = false;
            j+=i;
        }
    }
}