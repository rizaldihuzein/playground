mod euler_plus;
mod random_stuff;

use random_stuff::find_digit;
use euler_plus::*;

fn main() {
    f1_fizz_buzz_mult::f1();
    f2_even_fib_sum::f2();
    f3_large_prime_factor::f3();
    f4_palin::f4();
    f5_lcm_mult::f5();
    f6_diff_sqr::f6();
    f7_get_prime_nth::f7();
    f8_sum_of_consec::f8();
    f9_special_pyth_triple::f9();
    f10_sum_of_primes::f10();
    f11_square_4cons::f11();
    f12_div_tri::f12();
    f13_large_sum::f13();
    f14_longest_collatz::f14();
    f15_lattice_path::f15();
    find_digit::find_digit();
    f16_big_num_sum_digit::f16();
}