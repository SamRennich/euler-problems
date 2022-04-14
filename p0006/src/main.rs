/*
Problem 6: Sum Square Difference

The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385
The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 3025
Hence the difference between the sum of the squares of the
first ten natural numbers and the square of the sum is .
3025 - 385 = 2640
Find the difference between the sum of the squares of the
first one hundred natural numbers and the square of the sum.

Answer: 25164150
*/

const RANGE: i32 = 100;

fn main() {
    let mut squares_summed = 0;
    for i in 1..=RANGE {
        squares_summed += i * i;
    }

    let mut sum_squared = 0;
    for i in 1..=RANGE {
        sum_squared += i;
    }
    sum_squared *= sum_squared;

    println!("{}", sum_squared - squares_summed);
}
