/*
Problem 4: Largest Palindrome Product

A palindromic number reads the same both ways. 
The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.

Answer: 906609
Solved: May 24, 2021
*/

#include <iostream> // std
#include <cmath> // pow()

// Constants
const int DIGITS = 3;
const int RANGE = pow(10, DIGITS);

// Main program
int main() {
	
	int largestPalindrome = 0; // Tracks largest palindrome
	
	for (int i = 0; i < RANGE; i++) { // Iterates first number
		for (int j = 0; j < RANGE; j++) { // Iterates second number
			
			int n = i * j; // Finds product
			std::string nString= std::to_string(n); // Converts to string
			
			// Reverses string
			std::string reverse; // String to catch the reversed chars
			for (int i = (nString.length() - 1); i >= 0; i--) { // Iterate through every char, from back to front
				reverse.push_back(nString[i]); // Add to reversed string
			}
			
			if (nString == reverse && n > largestPalindrome) { // Checks if palindrome and greatest...
				largestPalindrome = n; // Stores number if so
			}
		}
	}
	
	std::cout << largestPalindrome << std::endl; // Print answer
	
	//Exit
    getchar();
    return 0;
}