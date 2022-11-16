"""
Ben Kahan  
Homework 9  
DS210  
16 November 2022  
Collaborators: none  
"""

import numpy as np

rng = np.random.default_rng()
tmp = rng.integers(low = -100000000,high = 100000000, size=1000)

file = open("data.txt", "w")
with open("data.txt", "w") as file:
    for num in range(len(tmp)): 
        file.write(str(tmp[num]))
        file.write('\n')