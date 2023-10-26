use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

/*
map checked bool n+1
ans = 0
for i = 2; i < n+1; i ++{
  if checked[i]{continue}

  checked[i] = true
  setsome
  for a = i*i,b = 2 ; a<=n ; b++ , a*=a{
    checked[a] = true
    for ex = 2, itung = ex*b; ex <= n; ex++ , itung += b{
      if itung > n setsome.ins(itung)
    }
  }
  ans += n-1
}
*/

fn f(n: i64) -> i64 {
    let mut checked: HashMap<i64, bool> = HashMap::new();
    let mut ans: i64 = 0;
    let mut exponent_set: HashSet<i64>;
    let mut a: i64;
    let mut b: i64;
    let mut deepexp: i64;

    for i in 2..n + 1 {
        if match checked.get(&i) {
            Some(v) => *v,
            None => false,
        } == true
        {
            continue;
        }
        exponent_set = HashSet::new();
        a = i * i;
        b = 2;
        while a <= n {
            checked.insert(a, true);

            deepexp = b;
            for multiplier in 2..n + 1 {
                if deepexp * multiplier > n {
                    exponent_set.insert(deepexp * multiplier);
                }
            }
            a *= i;
            b += 1;
        }
        ans += n - 1 + exponent_set.len() as i64;
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    let res = f(n);
    println!("{}", res);
}

pub fn f29(){
    main();
}
