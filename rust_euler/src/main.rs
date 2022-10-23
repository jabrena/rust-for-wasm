extern crate reikna;

use reikna::factor::quick_factorize;

// https://projecteuler.net/problem=1
// Problem 1: Multiples of 3 or 5 
// If we list all the natural numbers below 10 that are multiples of 3 or 5, 
// we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

pub fn euler1(param: i32) -> i32 {
    let mut sum = 0;

    for n in 1..param {
        if (n % 3 == 0) || (n % 5 == 0) {
            sum += n;
        }
    }

    return sum;
}

// https://projecteuler.net/problem=2
// Problem 2: Even Fibonacci numbers
// Each new term in the Fibonacci sequence is generated by adding 
// the previous two terms. By starting with 1 and 2, 
// the first 10 terms will be:
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// By considering the terms in the Fibonacci sequence whose values 
// do not exceed four million, find the sum of the even-valued terms.

fn fibonacci(n: usize) -> Vec<i64> {
    let mut list = vec![1, 1];
    for i in 2..n + 1 {
        let next_x = list[i - 1] + list[i - 2];
        list.push(next_x)
    }
    return list
}

pub fn euler2(param: i64) -> i64 {
    let list = fibonacci(param.try_into().unwrap());
    
    return list.iter()
        .filter(|n| *n % 2 == 0)
        .fold(0, |sum, &val| sum + val);
}

// https://projecteuler.net/problem=3
// Problem 3: Largest prime factor
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

pub fn euler3(param: i64) -> u64 {//TODO convert to i64

    let fac = quick_factorize(param.try_into().unwrap());
    println!("Factors: {:?}",fac);

    return *fac.last().unwrap();
}

// https://projecteuler.net/problem=4
// Problem 4 :Largest palindrome product
// A palindromic number reads the same both ways. 
// The largest palindrome made from the product 
// of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

pub fn euler4(param: i64) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn shuld_pass_euler1() {
        assert_eq!(euler1(10), 23);
        assert_eq!(euler1(1000), 233168);
    }

    #[test]
    fn shuld_pass_euler2() {
        assert_eq!(euler2(10), (2 + 8 + 34));
        //assert_eq!(euler2(4000000), 4613732);
    }

    #[test]
    fn shuld_pass_euler3() {
        assert_eq!(euler3(13195), 29);
        assert_eq!(euler3(600851475143), 6857);
    }

    //#[test]
    fn shuld_pass_euler4() {
        assert_eq!(euler4(2), 9009);
        assert_eq!(euler4(3), 906609);
    }
}