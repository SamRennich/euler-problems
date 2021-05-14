 /*
Problem 2: Even Fibonacci Numbers

Each new term in the Fibonacci sequence is generated by adding the previous two terms. 
By starting with 1 and 2, the first 10 terms will be:
1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
By considering the terms in the Fibonacci sequence whose values do not exceed four million, 
find the sum of the even-valued terms.

Answer: 4613732
Solved: Apr 5, 2021
*/

#include <iostream>

// Declare constants
const int RANGE = 4000000;

// Main program
int main() {
	
	int a = 1; // First integer
	int b = 1; // Second integer
	int sum = 0; // Sum tracker
	
	while (b <= RANGE) { // Iterate through all Fibonacci numbers through the range
		
		int temp = a; // Store abort
		a = b; // Swap
		b = b + temp; // Find new Fibonacci number
		
		if (b % 2 == 0) { // If even...
			sum += b; // Add to sum
		}
	}
	
	std::cout << sum << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}