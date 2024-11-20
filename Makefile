# Default variables
ROOT_DIR := $(shell pwd)
RISC0_DIR := $(ROOT_DIR)/risc0_benchmarks
SP1_DIR := $(ROOT_DIR)/sp1_benchmarks
RESULTS_DIR := $(ROOT_DIR)/results

# Target for RISC Zero project
.PHONY: risc0
risc0:
	@if [ -z "$(TEST_NAME)" ]; then \
        echo "Error: Please specify a TEST_NAME variable. Example: make risc0 PROJECT=test_project TEST_NAME=test_name"; \
        exit 1; \
    fi
	@echo "Running RISC Zero benchmarks for: TEST_NAME"
	@mkdir -p $(RESULTS_DIR)
	@cd $(RISC0_DIR)/test_project/methods && cargo build --release 
	@cd $(RISC0_DIR)/test_project/host && cargo build --release
	@cd $(RISC0_DIR)/test_project/host && RUST_LOG=info cargo run --release > $(RESULTS_DIR)/risc0_test_project_results.txt
	@echo "RISC Zero benchmarks completed! Results saved to $(RESULTS_DIR)/risc0_test_project_results.txt"

# Target for SP1 benchmarks
.PHONY: sp1
sp1:
	@echo "Running SP1 benchmarks..."
	@mkdir -p $(RESULTS_DIR)
	@bash $(SP1_DIR)/run_benchmark.sh > $(RESULTS_DIR)/sp1_results.txt
	@echo "SP1 benchmarks completed! Results saved to $(RESULTS_DIR)/sp1_results.txt"

# Run all benchmarks
.PHONY: all
all: risc0 sp1
	@echo "All benchmarks completed!"

