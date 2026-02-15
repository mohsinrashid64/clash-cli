// Benchmark: Generate 2M numbers, sort them, compute the sum.

fn main() {
    const N: usize = 2_000_000;

    // Generate numbers using Park-Miller LCG (works identically across languages)
    let mut numbers: Vec<i64> = Vec::with_capacity(N);
    let mut seed: i64 = 42;
    for _ in 0..N {
        seed = (seed * 48271) % 2147483647;
        numbers.push(seed);
    }

    // Sort
    numbers.sort_unstable();

    // Compute sum
    let total: i64 = numbers.iter().sum();

    println!("Sorted {} numbers. Sum = {}", N, total);
}
