 /*
Problem 3: Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?

Answer: 6857
Solved: 4/6/21
*/

#include <iostream>

// Declare constants
const long long RANGE = 600851475143;

// Main program
int main() {

	int divisor = 3; // Tracks the current divisor
	long long num = RANGE; // Sets the initial value of num
	int largestFactor = 0; // Tracks the largest prime factor

	while (num % 2 == 0) { // Iterates through evens
		num /= 2;
		largestFactor = 2;
	}

	while (num > 1) { // Iterates until value is fully factored
		if (num % divisor == 0) {
			num /= divisor;
			largestFactor = divisor;
		} else {
			divisor += 2; // Iterates through every odd divisor
		}
	}

	std::cout << largestFactor << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}