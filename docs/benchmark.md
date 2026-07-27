# Benchmarking

TechScript provides benchmarking utilities to measure execution performance and track speed regressions.

---

## 🏗️ Writing Benchmarks
Use the `benchmark` module to run functions and measure elapsed times:

```txs
use benchmark

# Define a function to benchmark
do compute_factorial()
    val = 1
    for i in 1..100000
        val = val * i
    end
end

# Run the benchmark: benchmark.run(name, iterations, function)
benchmark.run("Factorial loop 100k", 100, compute_factorial)
```

---

## 🚀 Running Benchmarks
Execute your benchmark script:
```bash
tech run perf.txs
```

Output:
```
Benchmark: Factorial loop 100k
  Iterations: 100
  Mean execution time: 2.15 ms
  Min time: 2.01 ms
  Max time: 2.45 ms
```

---

## 📊 System Benchmarking Command
You can also benchmark standard code snippets directly from the CLI:
```bash
tech benchmark "say 10 + 10" --iterations 1000
```
This runs the VM in isolation to output microsecond-accurate telemetry statistics.
