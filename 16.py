# Problem 16: Power Digit Sum

# 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
# What is the sum of the digits of the number 2^1000?

# Answer: 1366
# Solved: 4/8/21

num = str(pow(2, 1000)) # Finds the original value
sum = 0 # Tracks the sum total

for digit in num: # Iterates through every digit
    sum += int(digit) # Adds each digit to the sum

print(sum) # Prints answer