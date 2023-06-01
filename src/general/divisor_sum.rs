pub fn get_factor_sum(n:u64)->u64{
    let mut sum:u64 = 1;
    let mut i:u64 = 2;
    while i*i<=n{
        if n%i==0{
            sum+=i;
            if i*i!=n{
                sum+= n/i;
            }
        }
        i+=1;
    }
    sum
}