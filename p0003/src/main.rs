/*
Problem 3: Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?

Answer: 6857
*/

#[path = "../../modules/src/prime.rs"]
pub mod prime;

const NUM: usize = 600851475143;

fn main() {
	println!("{}", prime::prime_factors(NUM).last().unwrap().0);
}
