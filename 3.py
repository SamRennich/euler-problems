# Problem 3: Largest Prime Factor

# The prime factors of 13195 are 5, 7, 13 and 29.
# What is the largest prime factor of the number 600851475143?

# Answer: 6857
# Solved: 4/6/21

from math import sqrt

divisor = 3 # Current highest divisor
num = 600851475143

largestFactor = 0 # Tracks largest prime factor

while num % 2 == 0: # Iterates through evens
    num /= 2
    largestFactor = 2

while num > 1: # Iterates until value is fully factored
    if num % divisor == 0:
        num /= divisor
        largestFactor = divisor
        divisor -= 2
    divisor += 2 # Iterates through every odd divisor

print(largestFactor) # Prints answer