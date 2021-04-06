# Problem 6: Sum Square Difference

# The sum of the squares of the first ten natural numbers is,
# 1^2 + 2^2 + ... + 10^2 = 385
# The square of the sum of the first ten natural numbers is,
# (1 + 2 + ... + 10)^2 = 3025
# Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is .
# 3025 - 385 = 2640
# Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

# Answer: 25164150
# Solved: 4/6/21

sumSquare = 0 # Tracks the sum of squares
squareSum = 0 # Tracks the sum to be squared

for x in range(1, 101): # Sum of squares
    sumSquare += x * x

for y in range(1, 101): # Sum to be squared
    squareSum += y

squareSum *= squareSum # Sum gets squared

print(squareSum - sumSquare) # Prints answer