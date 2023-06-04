pub fn gen_fibonacci(n:u64,vec:&mut Vec<u64>){
    if vec.len() == 0{
        vec.push(1);
        vec.push(1);
    }
    for i in vec.len()..=n as usize{
        vec.push(vec[i]+vec[i-1]);
    }
}


// https://artofproblemsolving.com/wiki/index.php/Binet%27s_Formula
// binet's formula to calc approximation of fib-n
// approx_fib_n == 1/sqrt(5) * [(([1+sqrt[5]]/2)^n - ([1-sqrt[5]]/2)^n)]
// r_approx_fib_n ~= ([1+sqrt(5)]/2)^n / sqrt(5)
// we can use ceil(log10(approx_fib_n)) to approx the digit
// log10(r_approx_fib_n) = n*log10(1+sqrt(5)]/2) - 1/2*log10(5)
pub fn fib_digit(n:u64)->u64{
    let sqrt5 = 5_f64.sqrt();
    let log10_sqrt5 = sqrt5.log10();
    let tetha = (1_f64+sqrt5)/2_f64;
    ((n as f64 * tetha.log10()) - (5_f64.log10()/2_f64)).ceil() as u64
}