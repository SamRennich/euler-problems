/*
Problem 20: Factorial Digit Sum

n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

Answer: ???
*/

fn main() {
	let sum_of_digits = |num: u128| {
		println!("{}", num);
		let mut sum = 0;
		for digit in num.to_string().chars() {
			sum += digit.to_digit(10).unwrap();
		}
		sum
	};

	println!("{}", sum_of_digits(condensed_fac(38)));
}

fn condensed_fac(num: u128) -> u128 {
	if num > 1 {
		let mut term = num * condensed_fac(num - 1);

		while term % 10 == 0{
			term /= 10;
		}

		return term;
	} else {
		return num;
	}
}
