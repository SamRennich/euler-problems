/*
Problem 4: Largest Palindrome Product

A palindromic number reads the same both ways.
The largest palindrome made from the product of
two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.

Answer: 906609
*/

const DIGITS: u32 = 3;
const LIMIT: i32 = i32::pow(10, DIGITS);

fn main() {
	let rev = |num: i32| {
		num.to_string()
			.chars()
			.rev()
			.collect::<String>()
			.parse::<i32>()
			.unwrap()
	};

	let mut largest_palidrome = 0;

	for i in 1..LIMIT {
		for j in 1..LIMIT {
			let product = i * j;
			if product > largest_palidrome && product == rev(product) {
				largest_palidrome = product;
			}
		}
	}

	println!("{}", largest_palidrome);
}
