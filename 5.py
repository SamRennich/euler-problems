# Problem 5: Smallest Multiple

# 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
# What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

# Answer: 232792560
# Solved: 4/6/21

num = 1 # Tracks number

for x in range(1, 21): # Iterates 1 through 20
    step = num # Resets step value
    while num % x != 0:
        num += step # Iterates until divisble by next number in 1-20

print(num) # Prints answer