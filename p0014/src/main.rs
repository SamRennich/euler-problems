/*
Problem 14: Longest Collatz Sequence

The following iterative sequence is defined for the set of positive integers:
n → n/2 (n is even)
n → 3n + 1 (n is odd)
Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1)
contains 10 terms.
Although it has not been proved yet (Collatz Problem),
it is thought that all starting numbers finish at 1.
Which starting number, under one million, produces the longest chain?
NOTE: Once the chain starts the terms are allowed to go above one million.

Answer: 837799
*/

const RANGE: u64 = 1000000;

fn main() {
    let mut longest_chain_length = 0;
    let mut longest_chain_value = 0;

    for i in 0..RANGE {
        let mut num = i;
        let mut chain = 0;
        while num > 1 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num = 3 * num + 1;
            }
            chain += 1;
        }

        if chain > longest_chain_length {
            longest_chain_length = chain;
            longest_chain_value = i;
        }
    }

    println!("{}", longest_chain_value);
}
