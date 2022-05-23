/*
Problem 5: Smallest Multiple

2520 is the smallest number that can be divided by each of
the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evenly divisible
by all of the numbers from 1 to 20?

Answer: 232792560
*/

#[path = "../../modules/src/prime.rs"]
pub mod prime;

const LIMIT: usize = 20;

fn main() {
	let remainder = |mut base: Vec<usize>, removal| {
		for factor in removal {
			let index = base.iter().position(|&x| x == factor);
			if index != None {
				base.swap_remove(index.unwrap());
			}
		}

		base
	};

	let mut prime_factors = vec![];
	for i in 1..=LIMIT {
		prime_factors.push(prime::prime_factors_list(i));
	}

	let mut answer = vec![];
	for factor_list in prime_factors {
		answer.extend_from_slice(&remainder(factor_list, answer.clone()));
	}

	println!("{}", answer.iter().product::<usize>());
}
