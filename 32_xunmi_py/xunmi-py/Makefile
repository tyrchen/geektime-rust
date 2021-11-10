# 如果你的 BUILD_DIR 不同，可以 make BUILD_DIR=<your-dir>
BUILD_DIR := target/release

SRCS := $(wildcard src/*.rs) Cargo.toml
NAME = xunmi
TARGET = lib$(NAME)
BUILD_FILE = $(BUILD_DIR)/$(TARGET).dylib
BUILD_FILE1 = $(BUILD_DIR)/$(TARGET).so
TARGET_FILE = $(NAME)/$(NAME).so

all: $(TARGET_FILE)

test: $(TARGET_FILE)
	python3 -m pytest

$(TARGET_FILE): $(BUILD_FILE1)
	@cp $(BUILD_FILE1) $(TARGET_FILE)

$(BUILD_FILE1): $(SRCS)
	@cargo build --release
	@mv $(BUILD_FILE) $(BUILD_FILE1) || true

PHONY: test all
