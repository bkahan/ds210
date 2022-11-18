"""
Ben Kahan  
Homework 9
DS210  
16 November 2022  
Collaborators: none
"""

import numpy as np
import random

rng = np.random.default_rng()
tmp = rng.integers(low = 0,high = 100, size=1000)


file = open("src/data.txt", "w")
with open("src/data.txt", "w") as file:
    for num in range(len(tmp)):
        k = random.randint(0,1)
        file.write(str(tmp[num]) + " " + str(k))
        file.write('\n')