pub fn is_prime(n: usize) -> bool {
	if n <= 1 {
		return false;
	} else {
		let limit = (n as f64).sqrt() as usize;
		let sqrt_index = ((limit as f64).sqrt() as usize) / 2;
		let half_limit = (limit + 1) / 2;
		let mut bools = vec![true; half_limit];

		let square = |num| 2 * (num * num + num);
		let actual = |num| 2 * num + 1;

		for i in 1..=sqrt_index {
			if bools[i] {
				if n % actual(i) == 0 {
					return false;
				}

				for j in (square(i)..half_limit).step_by(actual(i)) {
					bools[j] = false;
				}
			}
		}

		for i in (sqrt_index + 1)..half_limit {
			if bools[i] {
				if n % actual(i) == 0 {
					return false;
				}
			}
		}

		return true;
	}
}

pub fn primes(limit: usize) -> Vec<usize> {
	if limit <= 1 {
		return vec![];
	} else {
		let sqrt_index = ((limit as f64).sqrt() as usize) / 2;
		let half_limit = (limit + 1) / 2;
		let mut bools = vec![true; half_limit];

		let square = |num| 2 * (num * num + num);
		let actual = |num| 2 * num + 1;

		let mut primes = vec![2];

		for i in 1..=sqrt_index {
			if bools[i] {
				for j in (square(i)..half_limit).step_by(actual(i)) {
					bools[j] = false;
				}
				primes.push(actual(i));
			}
		}

		for i in (sqrt_index + 1)..half_limit {
			if bools[i] {
				primes.push(actual(i));
			}
		}

		return primes;
	}
}

pub fn num_primes(limit: usize) -> usize {
	return primes(limit).len();
}

fn check_factor(div: usize, n: &mut usize, factors: &mut Vec<(usize, usize)>) {
	let mut factor = (div, 0);
	while *n % factor.0 == 0 {
		*n /= factor.0;
		factor.1 += 1;
	}
	if factor.1 > 0 {
		factors.push(factor);
	}
}

pub fn prime_factors(mut n: usize) -> Vec<(usize, usize)> {
	if n <= 1 {
		return vec![];
	} else if is_prime(n) {
		return vec![(n, 1)];
	} else {
		let mut prime_factors = vec![];

		check_factor(2, &mut n, &mut prime_factors);

		let mut i = 3;
		while n > 1 {
			check_factor(i, &mut n, &mut prime_factors);
			i += 2;
		}

		return prime_factors;
	}
}

pub fn num_unique_prime_factors(limit: usize) -> usize {
	return prime_factors(limit).len();
}

pub fn num_total_prime_factors(limit: usize) -> usize {
	let mut total = 0;

	for factor in prime_factors(limit) {
		total += factor.1;
	}

	return total;
}
