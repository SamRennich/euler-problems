# Problem 4: Largest Palindrome Product

# A palindromic number reads the same both ways. 
# The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
# Find the largest palindrome made from the product of two 3-digit numbers.

# Answer: 906609
# Solved: 4/6/21

largest = 0 # Tracks largest answer

for x in range(100, 999): # First integer iterations
    for y  in range(100, 999): # Second integer iterations
        if str(x * y) == (str(x * y)[::-1]) and x * y > largest: # Checks if palindrome and largest
            largest = x * y

print(largest) # Print answer