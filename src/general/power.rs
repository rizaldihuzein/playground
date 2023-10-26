pub fn power(mut a:u64, mut n:i64)->u64{
    let mut ans:u64 = 1;

    while n > 0{
        if n & 1 > 0{
            ans *= a;
        }
        a *= a;
        n = n >> 1;
    }

    ans
}