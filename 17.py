# Problem 17: Number Letter Counts

# If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
# If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
# NOTE: Do not count spaces or hyphens. 
# For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. 
# The use of "and" when writing out numbers is in compliance with British usage.

# Answer: 
# Solved: 4/ /21

charSum = 0 # Tracks the total number of characters

for i in range(342, 343): # Iterates through all numbers
    
    ones = int(i % 10) # Finds ones place digit
    tens = int(i / 10) % 10 # Finds tens place digit
    hundreds = int(i / 100) # Finds hundreds place digit
    
    # Finds characters for ones place
    if tens != 1:
        if ones == 1: charSum += 3
        elif ones == 2: charSum += 3
        elif ones == 3: charSum += 5
        elif ones == 4: charSum += 4
        elif ones == 5: charSum += 4
        elif ones == 6: charSum += 3
        elif ones == 7: charSum += 5
        elif ones == 8: charSum += 5
        elif ones == 9: charSum += 4
    
    # Finds characters for tens place
    if tens == 1: # For values 11-19 (due to nuances)
        if ones == 1: charSum += 6
        elif ones == 2: charSum += 6
        elif ones == 3: charSum += 8
        elif ones == 4: charSum += 8
        elif ones == 5: charSum += 7
        elif ones == 6: charSum += 7
        elif ones == 7: charSum += 9
        elif ones == 8: charSum += 8
        elif ones == 9: charSum += 8
    elif tens == 2: charSum += 6
    elif tens == 3: charSum += 6
    elif tens == 4: charSum += 5
    elif tens == 5: charSum += 5
    elif tens == 6: charSum += 5
    elif tens == 7: charSum += 7
    elif tens == 8: charSum += 6
    elif tens == 9: charSum += 6
    
    # Finds characters for hundreds place
    if hundreds == 1: charSum += 10
    elif hundreds == 2: charSum += 10
    elif hundreds == 3: charSum += 12
    elif hundreds == 4: charSum += 11
    elif hundreds == 5: charSum += 11
    elif hundreds == 6: charSum += 10
    elif hundreds == 7: charSum += 12
    elif hundreds == 8: charSum += 12
    elif hundreds == 9: charSum += 11
    
    if hundreds > 0 and (ones > 0 or tens > 10): # To account for the "and"
        charSum += 3

# charSum += 11 # For "one thousand"

print(charSum) # Prints answer