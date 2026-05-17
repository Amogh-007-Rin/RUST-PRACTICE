import numpy as np
import time

start_time = time.time()

# Create a vectorized array from 0 to 1B and sum it
# We use int64 because the result is larger than a standard 32-bit integer max
limit = 1000000000
sum_val = np.arange(limit + 1, dtype=np.int64).sum()

end_time = time.time()

print(f"Sum: {sum_val}")
print(f"Time taken: {end_time - start_time:.6f} seconds")