"""Used to check if a1 1f was calculated correctly"""

import numpy as np
from scipy.signal import convolve2d
print('Hello!')

# Define the input matrix
input_matrix = np.array([
    [1, 7, 6, 3, 6],
    [7, 6, 5, 6, 4],
    [5, 4, 7, 7, 0]
])

# Define the kernel
kernel = np.array([
    [1, 0, -1],
    [2, 0, -2],
    [1, 0, -1]
])

# Perform spatial convolution
output_matrix = convolve2d(input_matrix, kernel, mode='same', boundary='fill')

# Print the result
print(output_matrix)