# Default variables
ROOT_DIR := $(shell pwd)
RISC0_DIR := $(ROOT_DIR)/risc0_benchmarks
SP1_DIR := $(ROOT_DIR)/sp1_benchmarks
RESULTS_DIR := $(ROOT_DIR)/results



# Helper target to cleanup background processes
.PHONY: cleanup
cleanup:
	@rm -f $(RESULTS_DIR)/risc0_cpu_usage.log
	@rm -f $(RESULTS_DIR)/risc0_memory_leak.log
	@rm -f $(RESULTS_DIR)/risc0_rust_bench.log
	@rm -f $(RESULTS_DIR)/sp1_cpu_usage.log
	@rm -f $(RESULTS_DIR)/sp1_memory_leak.log
	@rm -f $(RESULTS_DIR)/sp1_rust_bench.log
	@-pkill -f "top -b -d 1" || true



# Target for RISC Zero project
.PHONY: risc0
risc0: cleanup
	@if [ -z "$(TEST_NAME)" ]; then \
        echo "Error: Please specify a TEST_NAME variable. Example: make risc0 TEST_NAME=test_name"; \
        exit 1; \
    fi
	@echo "Running RISC Zero benchmarks for: $(TEST_NAME)"
	@mkdir -p $(RESULTS_DIR)
	@cd $(ROOT_DIR)/tests && cargo run $(TEST_NAME)
	@cd $(RISC0_DIR)/test_project/methods && cargo build --release
	@cd $(RISC0_DIR)/test_project/host && cargo build --release
	@while true; do \
        top -b -d 1 -n 1 | head -n 5 >> $(RESULTS_DIR)/risc0_cpu_usage.log; \
        sleep 1; \
    done &
	

	@cd $(RISC0_DIR)/test_project/host && RUST_LOG=info valgrind --leak-check=full \
        --log-file=$(RESULTS_DIR)/risc0_memory_leak.log \
        ../target/release/host > $(RESULTS_DIR)/risc0_rust_bench.log
	@bash $(ROOT_DIR)/log_cleaner.sh -r $(RESULTS_DIR)/risc0_rust_bench.log -m $(RESULTS_DIR)/risc0_memory_leak.log -c \
		$(RESULTS_DIR)/risc0_cpu_usage.log -o $(RESULTS_DIR)/risc0_$(TEST_NAME)_benchmark_results.txt
	@$(MAKE) cleanup
	@echo "RISC Zero $(TEST_NAME) benchmarks completed! Results saved to $(RESULTS_DIR)/risc0_$(TEST_NAME)_benchmark_results.txt"



# Target for SP1 benchmarks
.PHONY: sp1
sp1: cleanup
	@if [ -z "$(TEST_NAME)" ]; then \
        echo "Error: Please specify a TEST_NAME variable. Example: make risc0 TEST_NAME=test_name"; \
        exit 1; \
    fi
	@echo "Running SP1 benchmarks for: $(TEST_NAME)"
	@mkdir -p $(RESULTS_DIR)
	@cd $(ROOT_DIR)/tests && cargo run $(TEST_NAME)
	@cd $(SP1_DIR)/sp1_project/program  && cargo prove build
	@while true; do \
        top -b -d 1 -n 1 | head -n 5 >> $(RESULTS_DIR)/sp1_cpu_usage.log; \
        sleep 1; \
    done &
	@cd $(SP1_DIR)/sp1_project && RUST_LOG=info target/release/fibonacci --prove > $(RESULTS_DIR)/sp1_rust_bench.log
	@bash $(ROOT_DIR)/log_cleaner.sh -r $(RESULTS_DIR)/sp1_rust_bench.log -m $(RESULTS_DIR)/sp1_memory_leak.log -c \
		$(RESULTS_DIR)/sp1_cpu_usage.log -o $(RESULTS_DIR)/sp1_$(TEST_NAME)_benchmark_results.txt
	@$(MAKE) cleanup
	@echo "SP1 benchmarks completed! Results saved to $(RESULTS_DIR)/sp1_results.txt"



# Run all benchmarks
.PHONY: all
all: risc0 sp1
	@echo "All benchmarks completed!"
