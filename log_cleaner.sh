#!/bin/bash

# Help function
print_usage() {
    echo "Usage: $0 -r RUST_BENCH_LOG -m MEMORY_LEAK_LOG -c CPU_USAGE_LOG -o OUTPUT_FILE"
    echo "Options:"
    echo "  -r : Path to rust benchmark log file"
    echo "  -m : Path to memory leak log file"
    echo "  -c : Path to CPU usage log file"
    echo "  -o : Path to output file"
    echo "  -h : Display this help message"
    exit 1
}

# Parse command line arguments
while getopts "r:m:c:o:h" opt; do
    case $opt in
        r) input_file_rust_bench="$OPTARG";;
        m) input_file_memory_leak="$OPTARG";;
        c) input_file_risc0_cpu="$OPTARG";;
        o) output_file="$OPTARG";;
        h) print_usage;;
        ?) print_usage;;
    esac
done

# Validate required arguments
if [ -z "$input_file_rust_bench" ] || [ -z "$input_file_memory_leak" ] || 
   [ -z "$input_file_risc0_cpu" ] || [ -z "$output_file" ]; then
    echo "Error: Missing required arguments"
    print_usage
fi

# Check if input files exist
for file in "$input_file_rust_bench" "$input_file_memory_leak" "$input_file_risc0_cpu"; do
    if [ ! -f "$file" ]; then
        echo "Error: File $file does not exist"
        exit 1
    fi
done

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

# Write to output file
{
    echo "Total cycles: $total_cycles"
    echo "Proving time: $proving_time"
    echo "Peak memory consumption during proving: $peak_memory_proving"
    echo "Proof size: $proof_size"
    echo "Verification time: $verification_time"
    echo "Peak memory consumption during verification: $peak_memory_verification"
    echo "Total memory leak: $total_memory_leak KB"
    echo "Total power consumption: $total_power_consumption units"
} > "$output_file"