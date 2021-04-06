# Problem 10: Summation of Primes

# The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
# Find the sum of all the primes below two million.

# Answer: 142913828922
# Solved: 4/6/21

from math import sqrt

sum = 2 # Sets sum tracker

for num in range(3, 2000000, 2): # Iterates through every odd number
    
    primeCheck = True # Default primality
    
    for x in range(3, int(sqrt(num) + 1), 2): # Checks if value is prime
        if num % x == 0:
            primeCheck =se
    
    if primeCheck: # If prime, adds to sum
        sum += num

print(sum) # Prints answer