#!/usr/bin/env bash
# Yulia benchmark harness.
#
# Methodology: docs/specs/2026-11-09-yulia-benchmark-methodology.md §4.
#   - 3 warm-up runs are discarded, then 10 measured runs are recorded.
#   - the median and population standard deviation are reported.
#   - `-march=native` is prohibited: it breaks reproducibility.
#
# Usage:
#   benchmarks/run.sh                run every registered kernel
#   benchmarks/run.sh <kernel>...    run only the named kernels
#
# A kernel is a `bash` script at benchmarks/kernels/<name>.sh. Each
# invocation performs exactly one measured run.

set -euo pipefail

# SOP-RESLIM: a single kernel run is bounded; a hung kernel fails the run
# instead of blocking it forever. Override with KERNEL_TIMEOUT=<seconds>.
KERNEL_TIMEOUT="${KERNEL_TIMEOUT:-300}"

# Clean up on interrupt so a killed run does not leave partial state.
trap 'echo "benchmark: interrupted" >&2; exit 130' INT TERM

WARMUP=3
RUNS=10

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
kernel_dir="$here/kernels"

if (($# > 0)); then
    kernels=("$@")
else
    kernels=()
    for path in "$kernel_dir"/*.sh; do
        [[ -e "$path" ]] || continue
        kernels+=("$(basename "$path" .sh)")
    done
fi

# Kernel names are interpolated into a path; reject anything that is not a
# plain name so a caller cannot escape the kernels directory.
for kernel in "${kernels[@]}"; do
    if [[ ! "$kernel" =~ ^[A-Za-z0-9_-]+$ ]]; then
        echo "benchmark: invalid kernel name '$kernel'" >&2
        exit 1
    fi
done

if ((${#kernels[@]} == 0)); then
    echo "benchmark: no kernels registered in $kernel_dir"
    echo "benchmark: add benchmarks/kernels/<name>.sh (see benchmarks/README.md)"
    exit 0
fi

# Milliseconds since the epoch. GNU date exposes `%N`; BSD date does not, so
# fall back to python3 and finally to whole seconds.
now_ms() {
    local ns
    ns="$(date +%s%N 2>/dev/null)"
    if [[ "$ns" =~ ^[0-9]+$ ]]; then
        echo $((ns / 1000000))
    elif command -v python3 >/dev/null 2>&1; then
        python3 -c 'import time; print(int(time.time() * 1000))'
    else
        echo $(( $(date +%s) * 1000 ))
    fi
}

# Prints the wall-clock milliseconds a single run of <kernel> took.
# Fails the whole run if the kernel exceeds KERNEL_TIMEOUT seconds.
elapsed_ms() {
    local start end
    start=$(now_ms)
    if ! timeout -k 5 "$KERNEL_TIMEOUT" bash "$kernel_dir/$1.sh" >/dev/null 2>&1; then
        echo "benchmark: kernel '$1' failed or exceeded ${KERNEL_TIMEOUT}s" >&2
        exit 1
    fi
    end=$(now_ms)
    echo $((end - start))
}

# Prints "median=<ms> stddev=<ms> n=<count>" for the given samples.
statistics() {
    printf '%s\n' "$@" | awk '
        { v[NR] = $1; sum += $1 }
        END {
            n = NR
            mean = sum / n
            for (i = 1; i <= n; i++) { d = v[i] - mean; var += d * d }
            var /= n
            for (i = 1; i <= n; i++)
                for (j = i + 1; j <= n; j++)
                    if (v[j] < v[i]) { t = v[i]; v[i] = v[j]; v[j] = t }
            median = (n % 2) ? v[(n + 1) / 2] : (v[n / 2] + v[n / 2 + 1]) / 2
            printf "median=%.1fms stddev=%.1fms n=%d\n", median, sqrt(var), n
        }'
}

# Record the environment so results are reproducible (spec §4.1).
echo "date: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
echo "host: $(uname -srm)"
if [[ -x ./target/release/yulia ]]; then
    echo "yulia: $(./target/release/yulia --version)"
else
    echo "yulia: ./target/release/yulia not built"
fi
echo

printf '%-16s %s\n' KERNEL RESULT
for kernel in "${kernels[@]}"; do
    if [[ ! -f "$kernel_dir/$kernel.sh" ]]; then
        echo "benchmark: unknown kernel '$kernel'" >&2
        exit 1
    fi

    for ((i = 0; i < WARMUP; i++)); do
        timeout -k 5 "$KERNEL_TIMEOUT" bash "$kernel_dir/$kernel.sh" >/dev/null 2>&1
    done

    samples=()
    for ((i = 0; i < RUNS; i++)); do
        samples+=("$(elapsed_ms "$kernel")")
    done

    printf '%-16s %s\n' "$kernel" "$(statistics "${samples[@]}")"
done
