// Benchmark: Generate 2M numbers, sort them, compute the sum.

const N = 2_000_000;

// Generate numbers using Park-Miller LCG (works identically across languages)
const numbers = new Array(N);
let seed = 42;
for (let i = 0; i < N; i++) {
    seed = (seed * 48271) % 2147483647;
    numbers[i] = seed;
}

// Sort numerically
numbers.sort((a, b) => a - b);

// Compute sum
let total = 0;
for (let i = 0; i < N; i++) {
    total += numbers[i];
}

console.log(`Sorted ${N} numbers. Sum = ${total}`);
