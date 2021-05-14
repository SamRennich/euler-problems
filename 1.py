# Problem 1: Multiples of 3 and 5

# If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 
# The sum of these multiples is 23.
# Find the sum of all the multiples of 3 or 5 below 1000.

# Answer: 233168
# Solved: 4/5/21

sum = 0 # Sum tracker

for i in range(99999999):
    if i % 3 == 0 or i % 5 == 0: # Divisible by 3 or 5
        sum += i # If so, add to sum

print(sum) # Prints answer