# Problem 9: Special Pythagorean Triplet

# A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
# a^2 + b^2 = c^2
# For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
# There exists exactly one Pythagorean triplet for which a + b + c = 1000.
# Find the product abc.

# Answer: 31875000
# Solved: 4/6/21

from math import sqrt

for a in range(1, 500): # Iterates through a
    for b in range(1, 500): # Iterates through b
        square  = a * a + b * b # Finds c^2
        if sqrt(square) == int(sqrt(square)) and a + b + int(sqrt(square)) == 1000: # Checks if correct
            answer = a * b * int(sqrt(square)) # Sets answer equal to correct value

print(answer) # Prints answer