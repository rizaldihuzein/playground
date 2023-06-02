pub fn gen_factorial(n:u64,vec:&mut Vec<u64>){
    if vec.len() == 0{
        vec.push(1);
    }
    for i in vec.len()..=n as usize{
        vec.push(i as u64 * vec[i-1])
    }
}