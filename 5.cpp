/*
Problem 5: Smallest Multiple

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

Answer: 232792560
Solved: May 24, 2021
*/

#include <iostream> // std

// Constants
const int RANGE = 20;

// Main program
int main() {
	
	unsigned long long int n = 1; // Tracks divisible number
	
	for (int i = 1; i <= RANGE; i++) { // Iterates through all values from 1 to RANGE
		unsigned long long int step = n; // Resets step to n every new iteration
		while (n % i != 0) { // Steps until divisible
			n += step;
		}
	}
	
	std::cout << n << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}