/*
Problem 1: Multiples of 3 and 5

If we list all the natural numbers below 10 that are multiples of 3 or 5,
we get 3, 5, 6 and 9.
The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.

Answer: 233168
*/

const LIMIT: i32 = 999;

fn main() {
	let sum = |limit| ((limit as f64 / 2.0) * (limit as f64 + 1.0)) as i32;

	let total = sum(LIMIT / 3) * 3 + sum(LIMIT / 5) * 5 - sum(LIMIT / 15) * 15;

	println!("{}", total);
}
