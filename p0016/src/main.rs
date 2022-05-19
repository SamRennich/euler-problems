/*
Problem 16: Power Digit Sum

2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
What is the sum of the digits of the number 2^1000?

Answer: 1366
*/

const RANGE: i32 = 1000;
const MULTIPLIER: u128 = 2;
const TRUNC: u128 = 10u128.pow(38);
const ARRAY_SIZE: usize = 8;

fn main() {
	let mut num: [u128; ARRAY_SIZE] = [0; ARRAY_SIZE];
	num[ARRAY_SIZE - 1] = 1;

	for _ in 0..RANGE {
		for i in 0..num.len() {
			num[i] *= MULTIPLIER;
		}

		for i in (1..num.len()).rev() {
			if num[i] > TRUNC {
				num[i] -= TRUNC;
				num[i - 1] += 1;
			}
		}
	}

	let mut sum = 0;

	for part in num {
		for digit in part.to_string().chars() {
			sum += digit.to_digit(10).unwrap();
		}
	}

	println!("{}", sum);
}
