SRC_DIR := src
BUILD_DIR := build
EVAL_DIR := eval
SRC_FILES := $(wildcard $(SRC_DIR)/*.rs)
SRC_FILES += $(wildcard $(SRC_DIR)/$(EVAL_DIR)/*.rs)
ROOT_FILE := $(SRC_DIR)/main.rs
OBJ_FILES := $(BUILD_DIR)/out.o $(BUILD_DIR)/syscall.o $(BUILD_DIR)/print_i64.o $(BUILD_DIR)/print_f64.o $(BUILD_DIR)/args.o

RUSTFLAGS := --edition=2021 -g # -Z threads=10

ifeq ($(RELEASE), 1)
	RUSTFLAGS += -C "opt-level=3"
endif

$(BUILD_DIR)/compiler: $(ROOT_FILE) $(SRC_FILES)
	/usr/bin/rustc -o $@ $(RUSTFLAGS) $<

$(BUILD_DIR)/out: $(BUILD_DIR)/out.s $(OBJ_FILES)
	/usr/bin/ld -o $@ $(OBJ_FILES)

$(BUILD_DIR)/%.o: %.s
	/usr/bin/as -o $@ $<

$(BUILD_DIR)/%.s: %.ssa
	/usr/bin/qbe $< > $@
