#!/bin/bash

# Help function
print_usage() {
  echo "Usage: $0 -r RUST_BENCH_LOG -c CPU_USAGE_LOG -o OUTPUT_FILE"
  echo "Options:"
  echo "  -r : Path to rust benchmark log file"
  echo "  -c : Path to CPU usage log file"
  echo "  -o : Path to output file"
  echo "  -h : Display this help message"
  exit 1
}

# Parse command line arguments
while getopts "r:c:o:h" opt; do
  case $opt in
  r) input_file_rust_bench="$OPTARG" ;;
  c) input_file_cpu_usage="$OPTARG" ;;
  o) output_file="$OPTARG" ;;
  h) print_usage ;;
  ?) print_usage ;;
  esac
done

# Validate required arguments
if [ -z "$input_file_rust_bench" ] ||
  [ -z "$input_file_cpu_usage" ] || 
  [ -z "$output_file" ]; then
  echo "Error: Missing required arguments"
  print_usage
fi

# Check if input files exist
for file in "$input_file_rust_bench" "$input_file_cpu_usage"; do
    if [ ! -f "$file" ]; then
        echo "Error: File $file does not exist"
        exit 1
    fi
done

# Extract total cycles - handles both "total cycles:" and "cycles=" formats
total_cycles=$(egrep "total cycles:| cycles=" "$input_file_rust_bench" |
  sed -E 's/.*((total cycles:| cycles=)\s*)([0-9]+).*/\3/' |
  sed 's/^[[:space:]]*//g' |
  head -n 1)

# Extract Proving time
proving_time=$(grep "^Proving time:" "$input_file_rust_bench" | awk '{print $3}')

# Extract Proof size
proof_size=$(grep "^Proof size:" "$input_file_rust_bench" | awk '{print $3, $4}')

# Extract Verification time
verification_time=$(grep "^Verification time:" "$input_file_rust_bench" | awk '{print $3}')

# Extract and clean CPU percentage (remove % and convert to integer)
avg_cpu_usage=$(grep "%Cpu(s)" "$input_file_cpu_usage" |
  awk '{for(i=1;i<=NF;i++) if($i ~ /us/) print $(i-1)}' |
  awk '{sum += $1} END {print int(sum/NR)}')

# Estimate the total power consumption during proving and verification by multiplying the average CPU usage with the time spent without using bc command
proving_time_clean=$(echo "$proving_time" | sed 's/s$//' | awk '{print int($1)}')
total_power_consumption=$((proving_time_clean * avg_cpu_usage))

peak_ram=$(grep "^MiB Mem" "$input_file_cpu_usage" | awk 'BEGIN {max=0; max_str=""} {
  used_str=$8;
  sub(",?$","",used_str);
  used_num=used_str;
  gsub(",",".",used_num);
  if (used_num+0 > max+0) {
    max=used_num;
    max_str=used_str
  }
} END {print max_str}')


# Write to output file
{
  echo "Total cycles: $total_cycles"
  echo "Proving time: $proving_time"
  echo "Proof size: $proof_size"
  echo "Verification time: $verification_time"
  echo "Total power consumption: $total_power_consumption units"
  echo "Peak RAM usage: $peak_ram MiB"
} >"$output_file"
