/*
Problem 7: 10001st Prime

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10001st prime number?

Answer: 104743
Solved: May 24, 2021
*/

#include <iostream> // std
#include <vector> // std::vector
#include <cmath> // sqrt()

// Constants
const int WHICH_PRIME = 10001; // Which prime to find
const int RANGE = 1000000; // Range of the prime seive

// Prototypes
void primeSeive(std::vector<bool>&); // Sets all non-prime values to false

// Main program
int main() {
	
	// Create initial vector
	std::vector<bool> primes(RANGE + 1, true);
	primes[0] = false; // Manual override for value 0
	primes[1] = false; // Manual override for value 1
	
	// Sort, leaving only primes true
	primeSeive(primes);
	
	int counter = 0; // Tracks how many primes have passed
	int i; // Saves last prime index
	
	for (i = 0; i < RANGE; i++) { // Checks through every number...
		if (primes[i]) { // And if prime...
			counter++; // Adds to counter
		}
		if (counter == WHICH_PRIME) { // If the prime is found...
			break; // Break loop
		}
	}
	
	std::cout << i << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}

// Removes all non-primes leaving only primes as true in the vector
void primeSeive(std::vector<bool>& primes) {
	
	unsigned long long int size = primes.size();
	
	// Sets scale to the square root of the RANGE for finding prime divisors
	unsigned long long int scale = static_cast<int>(sqrt(size - 1));
	
	// Turns all even numbers (bar 2) to false
	for (unsigned long long int i = 4; i < size; i += 2) {
		primes[i] = false;
	}
	
	// Iterates through every other value in the vector
	for (unsigned long long int i = 3; i <= scale; i += 2) {
		if (primes[i]) { // If the value is prime...
			for (unsigned long long int j = (i * 2); j < size; j += i) { // Set all multiples of it to be false
				primes[j] = false;
			}
		}
	}
	
	return; // Finish
}