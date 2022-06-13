/*
Problem 9: Special Pythagorean Triplet

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

Answer: 31875000
*/

const GOAL: i32 = 1000;

fn main() {
	let (mut m, mut n) = (0, 0);

	for i in (1..(((GOAL / 2) as f64).sqrt() as i32)).rev() {
		if (GOAL / 2 - i * i) % i == 0 {
			(m, n) = (i, (GOAL / 2 - i * i) / i);
			break;
		}
	}

	println!("{}", (m.pow(4) - n.pow(4)) * 2 * m * n)
}
