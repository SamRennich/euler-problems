# Problem 3: Largest Prime Factor

# The prime factors of 13195 are 5, 7, 13 and 29.
# What is the largest prime factor of the number 600851475143?

# Answer: 
# Solved: 

largestFactor = 0

num  = 600851474143

divisor = 2

while num > 1:
    if num % divisor == 0:
        num /= divisor
        
        if divisor > largestFactor:
            largestFactor = divisor
        
        divisor = 2
    else:
        divisor += 1

print(largestFactor)