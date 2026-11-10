# Benchmarks

Performance is a first-class requirement for Yulia, not an afterthought.
The authoritative methodology is
[`docs/specs/2026-11-09-yulia-benchmark-methodology.md`](../docs/specs/2026-11-09-yulia-benchmark-methodology.md).

## Running

```sh
make benchmark            # every registered kernel
benchmarks/run.sh k1      # one kernel
```

The harness discards 3 warm-up runs, records 10 measured runs, and reports the
median and population standard deviation in milliseconds.

## Adding a kernel

Create `benchmarks/kernels/<name>.sh`. It is invoked once per run and must
perform the measured work itself, printing nothing that the harness would
capture (output is discarded).

```sh
#!/usr/bin/env bash
# K1: matrix multiply, N=1024, float64.
set -euo pipefail
./target/release/yulia kernels/k1.yl
```

## Rules

- `-march=native` is **prohibited**: results must be reproducible across
  machines. Use `-march=x86-64-v3` (spec §4.1).
- Run on a quiet, dedicated machine; no other load (spec §4.1).
- Record the compiler version, flags, and machine in the result.
- A regression greater than 5% on any kernel blocks the change (spec §4.4).

## Kernels

| id | name | target | phase |
|---|---|---|---|
| K1 | matrix multiply, N=1024, float64 | within 5% of C | 1 |
| K2 | linked list, 1,000,000 nodes | within 10% of C | 4 |
| K3 | string processing, 10 MB file | within 15% of C | 4 |
| K4 | region allocation, 1,000,000 × 16-byte structs | ≥ 2× faster than C | 4 |
| K5 | compile time, 10,000-line program | ≥ 1.5× faster than rustc | 4 |

No kernels are registered yet; the harness is in place so the first kernel
lands without plumbing changes.
