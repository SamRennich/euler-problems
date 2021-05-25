/*
Problem 6: Sum Square Difference

The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385
The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is .
3025 - 385 = 2640
Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

Answer: 25164150
Solved: May 24, 2021
*/

#include <iostream> // std

// Constants
const int RANGE = 100;

// Main program
int main() {
	
	int sumSquare = 0; // Tracks the sum of sqaures
	for (int i = 1; i <= RANGE; i++) { // Adds the square of every number
		sumSquare += i * i;
	}
	
	int squareSum = 0; // Tracks the sum to be squared
	for (int i = 1; i <= RANGE; i++) { // Adds every number to the sum
		squareSum += i;
	}
	squareSum *= squareSum; // Squares the sum
	
	int difference = squareSum - sumSquare; // Finds the difference
	
	std::cout << difference << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}