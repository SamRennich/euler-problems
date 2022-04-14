/*
Problem 9: Special Pythagorean Triplet

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

Answer: 31875000
Solved: May 25, 2021
*/

#include <iostream> // std
#include <cmath> // sqrt()

// Constants
const int GOAL = 1000; // Stores the goal sum of the triplet
const int RANGE = GOAL / 2; // Sets the range of the search algorithm

// Main program
int main() {
	
	bool finishFlag = true; // Checks if the answer has been found. True if not, false if it has
	unsigned long long int finalProduct; // Stores the final answer
	
	for (int a = 1; a <= RANGE && finishFlag; a++) { // Iterates through all valid a values
		for (int b = 1; b <= RANGE && finishFlag; b++) { // Iterates through all valid b values
			
			int cSquared = (a * a) + (b * b); // Finds c^2
			int c = sqrt(cSquared); // Finds c
			
			if ((c * c) == cSquared && (a + b + c) == GOAL) { // Checks if valid and meets goal
				finalProduct = a *b * c; // Records answer
				finishFlag = false; // Closes flag to exit all loops
			}
		}
	}
	
	std::cout << finalProduct << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}