#!/bin/bash

# Input and output files
input_file_rust_bench="results/risc0_rust_bench.log"
input_file_memory_leak="results/risc0_memory_leak.log"
input_file_risc0_cpu="results/risc0_cpu_usage.log"
output_file="results/risc0_benchmark_results"

# Extract total cycles
total_cycles=$(grep "total cycles:" "$input_file_rust_bench" | awk '{print $NF}')

# Extract Proving time
proving_time=$(grep "^Proving time:" "$input_file_rust_bench" | awk '{print $3}')

# Extract Peak memory consumption during proving
peak_memory_proving=$(grep "^Peak memory consumption during proving:" "$input_file_rust_bench" | awk '{print $6, $7}')

# Extract Proof size
proof_size=$(grep "^Proof size:" "$input_file_rust_bench" | awk '{print $3, $4}')

# Extract Verification time
verification_time=$(grep "^Verification time:" "$input_file_rust_bench" | awk '{print $3}')

# Extract Peak memory consumption during verification
peak_memory_verification=$(grep "^Peak memory consumption during verification:" "$input_file_rust_bench" | awk '{print $6, $7}')

# Extract memory leak information
definitely_lost=$(grep "definitely lost:" "$input_file_memory_leak" | awk '{gsub(",", "", $4); print $4}')
indirectly_lost=$(grep "indirectly lost:" "$input_file_memory_leak" | awk '{gsub(",", "", $4); print $4}')
possibly_lost=$(grep "possibly lost:" "$input_file_memory_leak" | awk '{gsub(",", "", $4); print $4}')

# Sum up memory leaks in bytes
total_memory_leak=$(((definitely_lost + indirectly_lost + possibly_lost) / 1024))

# Extract and clean CPU percentage (remove % and convert to integer)
avg_cpu_usage=$(grep "%Cpu(s)" "$input_file_risc0_cpu" | 
    awk '{for(i=1;i<=NF;i++) if($i ~ /us/) print $(i-1)}' | 
    awk '{sum += $1} END {print int(sum/NR)}')

# Estimate the total power consumption during proving and verification by multiplying the average CPU usage with the time spent without using bc command
proving_time_clean=$(echo "$proving_time" | sed 's/s$//' | awk '{print int($1)}')
total_power_consumption=$((proving_time_clean * avg_cpu_usage))

# Extract total sum of CPU usage (without averaging)
total_cpu_usage=$(grep "%Cpu(s)" "$input_file_risc0_cpu" | 
    awk '{for(i=1;i<=NF;i++) if($i ~ /us/) print $(i-1)}' | 
    awk '{sum += $1} END {print int(sum)}')

# Write to output file
{
    echo "Total cycles: $total_cycles"
    echo "Proving time: $proving_time"
    echo "Peak memory consumption during proving: $peak_memory_proving"
    echo "Proof size: $proof_size"
    echo "Verification time: $verification_time"
    echo "Peak memory consumption during verification: $peak_memory_verification"
    echo "Total memory leak: $total_memory_leak KB"
    echo "Total power consumption: $total_cpu_usage%" >> "$output_file"
} > "$output_file"