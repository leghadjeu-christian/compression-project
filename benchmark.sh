#!/bin/bash

# --- Configuration ---
COMPRESS_CMD="your_compression_command"  # Replace with your compression command
DECOMPRESS_CMD="your_decompression_command" # Replace with your decompression command
INPUT_FILE="your_input_file"          # Replace with your input file
COMPRESSED_FILE="compressed_output.bin" # Output file for compression
DECOMPRESSED_FILE="decompressed_output" # Output file for decompression
NUM_RUNS=5                            # Number of times to run each operation

# --- Helper Function to Run and Time a Command ---
run_and_time() {
  local command="$1"
  local label="$2"
  local start_time
  local end_time
  local duration

  echo "--- Benchmarking: $label ---"

  start_time=$(date +%s.%N)
  eval "$command"
  end_time=$(date +%s.%N)
  duration=$(echo "$end_time - $start_time" | bc)
  echo "  Execution Time: $duration seconds"

  # Get output size (if applicable)
  if [[ "$label" == "Compression" ]]; then
    output_size=$(du -b "$COMPRESSED_FILE" | awk '{print $1}')
    original_size=$(du -b "$INPUT_FILE" | awk '{print $1}')
    if [[ -n "$output_size" && -n "$original_size" && "$original_size" -gt 0 ]]; then
      compression_ratio=$(echo "scale=2; (1 - ($output_size / $original_size)) * 100" | bc)
      echo "  Compressed Size: $output_size bytes"
      echo "  Original Size:   $original_size bytes"
      echo "  Compression Ratio: ${compression_ratio}%"
    fi
  elif [[ "$label" == "Decompression" ]]; then
    output_size=$(du -b "$DECOMPRESSED_FILE" | awk '{print $1}')
    compressed_size=$(du -b "$COMPRESSED_FILE" | awk '{print $1}')
    if [[ -n "$output_size" && -n "$compressed_size" ]]; then
      echo "  Decompressed Size: $output_size bytes"
      echo "  Compressed Size:   $compressed_size bytes"
    fi
  elif [[ "$label" == "Output" ]]; then
    output_size=$(du -b "$DECOMPRESSED_FILE" | awk '{print $1}')
    echo "  Output File Size: $output_size bytes"
  fi
  echo ""
}

# --- Run Benchmarks ---

echo "Starting benchmark with $NUM_RUNS runs for each operation..."
echo ""

# Compression Benchmark
for i in $(seq 1 "$NUM_RUNS"); do
  run_and_time "$COMPRESS_CMD \"$INPUT_FILE\" \"$COMPRESSED_FILE\"" "Compression (Run $i)"
done

# Decompression Benchmark
if [[ -f "$COMPRESSED_FILE" ]]; then
  for i in $(seq 1 "$NUM_RUNS"); do
    run_and_time "$DECOMPRESS_CMD \"$COMPRESSED_FILE\" \"$DECOMPRESSED_FILE\"" "Decompression (Run $i)"
  done
else
  echo "Warning: Compressed file '$COMPRESSED_FILE' not found. Skipping decompression benchmark."
fi

# Output Time (assuming decompression generates the final output)
if [[ -f "$DECOMPRESSED_FILE" ]]; then
  run_and_time "cat \"$DECOMPRESSED_FILE\" > /dev/null" "Output Time (Simulated)"
else
  echo "Warning: Decompressed file '$DECOMPRESSED_FILE' not found. Skipping output time measurement."
fi

echo "Benchmark complete."

# --- Cleanup (Optional) ---
# rm -f "$COMPRESSED_FILE" "$DECOMPRESSED_FILE"
