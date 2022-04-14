# Problem 15: Lattice Paths

# Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, 
# there are exactly 6 routes to the bottom right corner.
# How many such routes are there through a 20×20 grid?

# Answer: 137846528820
# Solved: 4/8/21

from math import pow

num = 2 # Starts the paths at the value of a 1x1 lattice
power = 0 # Default power value

for i in range(2, 21): # Iterates through all lattices
    num = (num + (num * ((1 + power) / (2 + power)))) * 2
    power += 1 # Increases power each iteration

print(int(num)) # Prints answer