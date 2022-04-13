/*
Problem 1: Multiples of 3 and 5

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.

Answer: 233168
*/

const RANGE: i32 = 999;

fn main() {

    let sum = sum(RANGE / 3) * 3 + sum(RANGE / 5) * 5 - sum(RANGE / 15) * 15;

    println!("{}", sum);
}

fn sum(range: i32) -> i32 {
    ((range as f64 / 2.0) * (range as f64 + 1.0)) as i32
}
