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

const LIMIT: i32 = 100;

fn main() {
	let squares_summed = (1..=LIMIT).map(|x| x * x).sum::<i32>();
	let sum_squared = (1..=LIMIT).sum::<i32>().pow(2);

	println!("{}", sum_squared - squares_summed);
}
