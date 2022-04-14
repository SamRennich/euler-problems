# Problem 14: Longest Collatz Sequence

# The following iterative sequence is defined for the set of positive integers:
# n → n/2 (n is even)
# n → 3n + 1 (n is odd)
# Using the rule above and starting with 13, we generate the following sequence:
# 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
# It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. 
# Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
# Which starting number, under one million, produces the longest chain?
# NOTE: Once the chain starts the terms are allowed to go above one million.

# Answer: 837799
# Solved: 4/7/21

longestChain = 0 # Tracks the longest chain

for i in range(1, 1000000): # Iterates through every number
    
    num = i
    currentChain = 1 # Tracks the current chain length
    
    while num > 1: # Iterates until chain is complete
        if num % 2 == 0: # If num is even
            num /= 2
        else: # If num is odd
            num = 3 * num + 1
        currentChain += 1
    
    if currentChain > longestChain: # Checks if chain is new longest
        longestChain = currentChain
        answer = i # Contains value of number with longest chain

print(answer) # Prints answer