TARGET := riscv64gc-unknown-none-elf
MODE := release
TARGET_DIR := target/$(TARGET)/$(MODE)

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

std:
	@cd std && make build

build:
	@cargo build --release 
	@$(OBJCOPY) --strip-all $(TARGET_DIR)/helium -O binary  $(TARGET_DIR)/helium.os 
