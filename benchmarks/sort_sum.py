"""Benchmark: Generate 2M numbers, sort them, compute the sum."""

N = 2_000_000

# Generate numbers using Park-Miller LCG (works identically across languages)
numbers = [0] * N
seed = 42
for i in range(N):
    seed = (seed * 48271) % 2147483647
    numbers[i] = seed

# Sort in place
numbers.sort()

# Compute sum
total = sum(numbers)

print(f"Sorted {N} numbers. Sum = {total}")
