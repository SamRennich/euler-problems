/*
Problem 10: Summation of Primes

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
Find the sum of all the primes below two million.

Answer: 142913828922
*/

const RANGE: u64 = 2000000;

fn main() {
    let primes = prime_data::PrimeData::generate(1..=RANGE);

    let mut sum = 0;
    for prime in primes.iter_all() {
        sum += prime;
    }

    println!("{}", sum);
}
