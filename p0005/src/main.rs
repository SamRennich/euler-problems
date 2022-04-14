/*
Problem 5: Smallest Multiple

2520 is the smallest number that can be divided by each of
the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

Answer: 232792560
*/

const RANGE: i32 = 20;

fn main() {
    let remainder = |mut base: Vec<u64>, removal: Vec<u64>| {
        for i in removal {
            let index = base.iter().position(|&x| x == i);

            if index != None {
                base.remove(index.unwrap());
            }
        }

        base
    };

    let mut prime_factors = vec![];
    for i in 1..=RANGE {
        prime_factors.push(primes::factors(i as u64));
    }

    let mut answer_factors = vec![];
    for factor_list in prime_factors {
        answer_factors.append(&mut remainder(factor_list, answer_factors.clone()));
    }

    let mut product = 1;
    for i in answer_factors {
        product *= i;
    }

    println!("{}", product);
}
