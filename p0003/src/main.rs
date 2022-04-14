/*
Problem 3: Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?

Answer: 6857
*/

const RANGE: u64 = 600851475143;

fn main() {
    let mut num = RANGE;
    let mut largest_prime_factor = 0;

    while num % 2 == 0 {
        num /= 2;
        largest_prime_factor = 2;
    }

    let mut i = 3;

    while num > 1 {
        while num % i == 0 {
            num /= i;
            largest_prime_factor = i;
        }
        i += 2;
    }

    println!("{}", largest_prime_factor);
}
