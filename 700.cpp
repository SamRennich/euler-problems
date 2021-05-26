/*
Problem 700: Eulercoin

Leonhard Euler was born on 15 April 1707.
Consider the sequence 1504170715041707n mod 4503599627370517.
An element of this sequence is defined to be an Eulercoin if it is strictly smaller than all previously found Eulercoins.
For example, the first term is 1504170715041707 which is the first Eulercoin. 
The second term is 3008341430083414 which is greater than 1504170715041707 so is not an Eulercoin. 
However, the third term is 8912517754604 which is small enough to be a new Eulercoin.
The sum of the first 2 Eulercoins is therefore 1513083232796311.
Find the sum of all Eulercoins.

Answer:
Solved:
*/

#include <iostream> // std

// Declare constants
const unsigned long long int COEFFICIENT = 1504170715041707; // COEFFICIENT for multiples
const unsigned long long int DIVISOR = 4503599627370517; // DIVISOR for modulo calculation

// Main program
int main() {
	
	unsigned long long int sum = 0; // Tracks sum
	unsigned long long int eulercoin = DIVISOR; // Stores the current Eulercoin
	
	for (int i = 1; eulercoin > 0; i++) { // Iterates until no more Eulercoins can be found
		
		unsigned long long int modResult = (i * COEFFICIENT) % DIVISOR; // Calculates remainder
		
		if (modResult < eulercoin) { // If new Eulercoin is found...
			eulercoin = modResult; // Set new Eulercoin
			std::cout << eulercoin << std::endl;
			sum += modResult; // Add to sum
		}
	}
	
	std::cout << sum << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}