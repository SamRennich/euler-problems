/*
Problem 1: Multiples of 3 and 5

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 
The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.

Answer: 233168
Solved: Apr 5, 2021
*/

#include <iostream>

// Declare constants
const int RANGE = 1000;

// Main program
int main() {
	
	int sum = 0; // Sum tracker
	
	for (int i = 0; i < RANGE; i++) { // Iterate through all values
		if (i % 3 == 0) { // Divisible by 3 or 5
			sum += i; // Add to counter
		} else if (i % 5 == 0) {
			sum += i; // Add to counter
		}
	}
	
	std::cout << sum << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}