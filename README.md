# ⚔️ clash

**Run commands head-to-head and compare their performance.**

`clash` is a CLI benchmarking tool that measures both **execution time** and **peak memory usage** — then shows you a beautiful side-by-side comparison. Think of it as [`hyperfine`](https://github.com/sharkdp/hyperfine) meets memory profiling.

## Why clash?

Most benchmarking tools only measure time. But performance isn't just speed — memory matters too. `clash` gives you the full picture:

- ⏱ **Time**: mean, min, max, and standard deviation across multiple runs
- 💾 **Memory**: peak RSS (resident set size) tracked in real-time during execution
- 📊 **Visual comparison**: colored bar charts right in your terminal
- 📁 **JSON export**: machine-readable results for CI pipelines or further analysis

---

## Quick Start

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (for building clash)
- Any commands you want to benchmark (e.g. `python`, `node`, etc.)

### Install

```bash
git clone https://github.com/mohsinrashid64/clash-cli.git
cd clash
cargo install --path .
```

That's it. `clash` is now available globally in your terminal.

### Try it instantly

No setup needed — benchmark any two commands you already have:

```bash
# Compare any two commands (works on any OS)
clash "echo hello" "echo world" --runs 10
```

---

## Demo: Python vs Node.js vs Rust

The repo includes identical benchmark scripts in three languages (generate 2M numbers, sort them, compute the sum). This is the best way to see clash in action.

### What you need

| Language | Required | Check with |
|----------|----------|------------|
| Python | Yes | `python --version` |
| Node.js | Yes | `node --version` |
| Rust | Already built | clash itself is Rust |

### Step 1: Compile the Rust benchmark

```bash
cd clash
rustc -O -o benchmarks/sort_sum_rust benchmarks/sort_sum.rs
```

> **Windows:** The output will be `benchmarks/sort_sum_rust.exe`

### Step 2: Run the 3-way comparison

**Linux / macOS:**
```bash
clash "python benchmarks/sort_sum.py" "node benchmarks/sort_sum.js" "benchmarks/sort_sum_rust" --runs 5
```

**Windows:**
```powershell
clash "python benchmarks/sort_sum.py" "node benchmarks/sort_sum.js" "benchmarks/sort_sum_rust.exe" --runs 5
```

You'll see Rust dominate on speed, Python use less memory than Node, and a clear summary of who wins what.

### Step 3: Try different comparisons

```bash
# Just Python vs Rust (biggest contrast)
clash "python benchmarks/sort_sum.py" "benchmarks/sort_sum_rust.exe" --runs 5

# With warmup runs and JSON export
clash "python benchmarks/sort_sum.py" "node benchmarks/sort_sum.js" --warmup 2 --runs 10 --export results.json
```

---

## More Examples You Can Run Right Now

These commands work out of the box — no extra files needed.

### Cross-platform (Windows, macOS, Linux)

```bash
# 1. Python one-liner: big list vs small list (shows memory difference)
clash "python -c \"x=list(range(5_000_000)); print(len(x))\"" "python -c \"x=list(range(1_000)); print(len(x))\"" --runs 3

# 2. Node.js: JSON parse vs regex (CPU-bound comparison)
clash "node -e \"JSON.parse(JSON.stringify({a:1,b:2,c:3})); console.log('done')\"" "node -e \"/\\d+/.test('hello123'); console.log('done')\"" --runs 10

# 3. Python vs Node startup time (who boots faster?)
clash "python -c \"print('hello')\"" "node -e \"console.log('hello')\"" --runs 10

# 4. Rust sort benchmark vs Python (requires compiled Rust benchmark)
clash "benchmarks/sort_sum_rust.exe" "python benchmarks/sort_sum.py" --runs 5

# 5. Export results and compare with warmup
clash "python -c \"sum(range(1_000_000))\"" "node -e \"let s=0;for(let i=0;i<1_000_000;i++)s+=i\"" --warmup 2 --runs 5 --export results.json
```

### macOS / Linux only

```bash
# 1. Compare shell interpreters
clash "bash -c 'for i in $(seq 1 10000); do :; done'" "zsh -c 'for i in $(seq 1 10000); do :; done'" --runs 5

# 2. curl vs wget
clash "curl -s -o /dev/null https://example.com" "wget -q -O /dev/null https://example.com" --runs 5

# 3. find vs fd (if fd is installed)
clash "find /usr -name '*.h' -type f" "fd -e h . /usr" --runs 3

# 4. grep vs ripgrep (if rg is installed)
clash "grep -r 'import' /usr/include --include='*.h' -l" "rg -l 'import' /usr/include -t h" --runs 5

# 5. Compression comparison
clash "gzip -k -f /tmp/clash_test.txt" "bzip2 -k -f /tmp/clash_test.txt" --runs 3
```

### Windows only

```powershell
# 1. PowerShell vs Python for simple math
clash "powershell -Command \"1..10000 | Measure-Object -Sum\"" "python -c \"print(sum(range(10000)))\"" --runs 5

# 2. Compare ping times to two hosts
clash "ping -n 2 127.0.0.1" "ping -n 2 1.1.1.1" --runs 3

# 3. dir listing via different methods
clash "cmd /C dir /s C:\Windows\System32\drivers" "powershell -Command \"Get-ChildItem C:\Windows\System32\drivers -Recurse\"" --runs 3

# 4. Python list comprehension vs loop
clash "python -c \"x=[i*2 for i in range(1_000_000)]; print(len(x))\"" "python -c \"x=[];\\nfor i in range(1_000_000): x.append(i*2)\\nprint(len(x))\"" --runs 5

# 5. Node crypto hashing
clash "node -e \"require('crypto').createHash('sha256').update('hello'.repeat(100000)).digest('hex')\"" "node -e \"require('crypto').createHash('md5').update('hello'.repeat(100000)).digest('hex')\"" --runs 10
```

---

## Example Output

```
  ⚔️  clash — benchmark comparator

  ✓ python benchmarks/sort_sum.py (5 runs)
  ✓ node benchmarks/sort_sum.js (5 runs)
  ✓ benchmarks/sort_sum_rust.exe (5 runs)

╭─────────┬────────────────────┬──────────────────────┬──────────────────────────╮
│ ⏱  Time │ python sort_sum.py │ node sort_sum.js     │ benchmarks/sort_sum_rust │
╞═════════╪════════════════════╪══════════════════════╪══════════════════════════╡
│ Mean    │ 1.842s             │ 0.523s               │ 0.098s                   │
│ Min     │ 1.801s             │ 0.511s               │ 0.095s                   │
│ Max     │ 1.899s             │ 0.539s               │ 0.102s                   │
│ Std Dev │ ±0.038s            │ ±0.011s              │ ±0.003s                  │
╰─────────┴────────────────────┴──────────────────────┴──────────────────────────╯
  python sort_sum.py         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  1.842s
  node sort_sum.js           ━━━━━━━━━─────────────────────  0.523s
  benchmarks/sort_sum_rust   ━━────────────────────────────  0.098s
  → benchmarks/sort_sum_rust is 18.80x faster

╭────────────┬────────────────────┬──────────────────────┬──────────────────────────╮
│ 💾  Memory │ python sort_sum.py │ node sort_sum.js     │ benchmarks/sort_sum_rust │
╞════════════╪════════════════════╪══════════════════════╪══════════════════════════╡
│ Peak RSS   │ 72.3 MB            │ 96.1 MB              │ 16.2 MB                  │
╰────────────┴────────────────────┴──────────────────────┴──────────────────────────╯
  benchmarks/sort_sum_rust   ━━━━━─────────────────────────  16.2 MB
  python sort_sum.py         ━━━━━━━━━━━━━━━━━━━━━━━───────  72.3 MB
  node sort_sum.js           ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  96.1 MB
  → benchmarks/sort_sum_rust uses 5.93x less memory

  Summary: benchmarks/sort_sum_rust wins on speed (18.80x), benchmarks/sort_sum_rust wins on memory (5.93x)
```

> *Note: Exact numbers will vary by machine. The relative differences are what matter.*

---

## CLI Reference

```
Usage: clash [OPTIONS] <COMMANDS> <COMMANDS>...

Arguments:
  <COMMANDS>...  Commands to benchmark (at least 2)

Options:
  -r, --runs <RUNS>      Number of benchmark runs per command [default: 5]
  -w, --warmup <WARMUP>  Number of warmup runs before benchmarking [default: 0]
  -e, --export <EXPORT>  Export results to JSON file
  -h, --help             Print help
  -V, --version          Print version
```

| Flag | What it does | Example |
|------|-------------|---------|
| `--runs 10` | Run each command 10 times for better statistics | `clash "cmd1" "cmd2" --runs 10` |
| `--warmup 3` | Run 3 untimed warmup iterations first (warms caches) | `clash "cmd1" "cmd2" --warmup 3` |
| `--export out.json` | Save results as JSON for CI or further analysis | `clash "cmd1" "cmd2" --export out.json` |

---

## How It Works

1. Each command is spawned as a child process with stdout/stderr suppressed
2. A monitoring thread polls the process every 30ms to track peak memory (RSS)
3. Wall-clock time is measured with `std::time::Instant`
4. After all runs complete, statistics are computed and displayed
5. Winners are highlighted in green; losers in red

## Compared to hyperfine

| Feature | clash | hyperfine |
|---------|-------|-----------|
| Execution time | ✅ | ✅ |
| Statistical analysis | ✅ | ✅ |
| **Peak memory tracking** | **✅** | ❌ |
| Warmup runs | ✅ | ✅ |
| JSON export | ✅ | ✅ |
| Shell overhead correction | ❌ | ✅ |
| Parameter scanning | ❌ | ✅ |

`clash` is focused on giving you the **full performance picture** — time + memory — in a single tool.

## Built With

- **Rust** — for speed and low overhead (a benchmark tool should be fast itself)
- **clap** — CLI argument parsing
- **sysinfo** — cross-platform process monitoring
- **comfy-table** — beautiful Unicode tables
- **owo-colors** — terminal colors
- **indicatif** — progress bars during benchmarks

## License

MIT
