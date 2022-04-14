/*
Problem 10: Summation of Primes

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
Find the sum of all the primes below two million.

Answer: 142913828922
Solved: May 25, 2021
*/

#include <iostream> // std
#include <vector> // std::vector
#include <cmath> // sqrt()

// Declare constants
const int RANGE = 2000000; // Sets the range of the prime sieve

// Prototypes
void primeSieve(std::vector<bool>&); // Sets all non-prime values to false

// Main program
int main() {
	
	// Create initial prime vector
	std::vector<bool> primes(RANGE + 1, true);
	primes[0] = false; // Manual override for value 0
	primes[1] = false; // Manual override for value 1
	
	// Sort, leaving only primes true
	primeSieve(primes);
	
	unsigned long long int sum = 0; // Tracks the sum of primes
	
	for (int i = 0; i < primes.size(); i++) { // Iterates through every value
		if (primes[i]) { // If prime...
			sum += i; // Add to sum
		}
	}
	
	std::cout << sum << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}

// Removes all non-primes leaving only primes as true in the vector
void primeSieve(std::vector<bool>& primes) {
	
	unsigned long long int size = primes.size(); // Finds size of primes vector
	
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