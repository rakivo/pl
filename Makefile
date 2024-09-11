SRC_DIR := src
BUILD_DIR := build
EVAL_DIR := eval
SRC_FILES := $(wildcard $(SRC_DIR)/*.rs)
SRC_FILES += $(wildcard $(SRC_DIR)/$(EVAL_DIR)/*.rs)
ROOT_FILE := $(SRC_DIR)/main.rs

RUSTFLAGS := --edition=2021 -g # -Z threads=10

ifeq ($(RELEASE), 1)
	RUSTFLAGS += -C "opt-level=3"
endif

$(BUILD_DIR)/compiler: $(ROOT_FILE) $(SRC_FILES)
	rustc -o $@ $(RUSTFLAGS) $<

$(BUILD_DIR)/main: $(BUILD_DIR)/main.o $(BUILD_DIR)/syscall.o $(BUILD_DIR)/print_i64.o
	ld -o $@ $^

$(BUILD_DIR)/%.o: %.s
	as -o $@ $<

$(BUILD_DIR)/%.s: %.ssa
	./qbe-1.2/qbe $< > $@
