/*
Problem 9: Special Pythagorean Triplet

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

Answer: 31875000
*/

use integer_sqrt::IntegerSquareRoot;

const GOAL: i32 = 1000;

fn main() {
    let (mut m, mut n): (i32, i32) = (0, 0);

    for i in 1..(GOAL / 2).integer_sqrt() {
        if (GOAL / 2 - i * i) % i == 0 {
            m = i;
            n = (GOAL / 2 - i * i) / i;
        }
    }

    println!("{}", (i32::pow(m, 4) - i32::pow(n, 4)) * 2 * m * n)
}
