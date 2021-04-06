# Problem 7: 10001st Prime

# By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
# What is the 10001st prime number?

# Answer: 104743
# Solved: 4/6/21

from math import sqrt

primeCount = 1 # Tracks the number of primes
num = 3 # Running num for prime check

while primeCount < 10001: # Iterates until 10001st prime is found
    
    primeCheck = True # Default primality
    
    for x in range(3, int(sqrt(num) + 1), 2): # Checks if value is prime
        if num % x == 0:
            primeCheck = False
    
    if primeCheck: # Counts up if value is prime
        primeCount += 1
    
    num += 2 # Iterates to next number

print(num - 2) # Prints answer