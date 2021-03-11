#!/usr/bin/make

# env
export PATH			:= /home/chipen/.cargo/bin/:$(PATH)

# toolchain
export shell		:= /bin/bash
export ECHO			:= echo
export EECHO		:= echo -e
export CD			:= cd
export RM			:= rm
export make			:= make
export RUSTC		:= rustc
export CARGO		:= cargo
export OBJCOPY		:= rust-objcopy
export QEMU			:= qemu-system-riscv64

# path
export ROOT			:= $(shell pwd)
export KERNEL		:= $(ROOT)/kernel
export USER			:= $(ROOT)/user
export BOOTLOADER	:= $(ROOT)/bootloader
export TYPE			?= release
export TYPE_DIR		:=

ifeq (${TYPE}, release)
    override TYPE := release
    override TYPE_DIR := release
else
    override TYPE := 
    override TYPE_DIR := debug
endif

export TYPE
export TYPE_DIR

# important files
FINAL				:= $(KERNEL)/target/riscv64gc-unknown-none-elf/$(TYPE_DIR)/os.bin
LIBUSER				:= $(USER)/target/riscv64gc-unknown-none-elf/$(TYPE_DIR)/libuser_lib.rlib
LINKER				:= $(KERNEL)/src/link_app.asm


# variable
TITLE_COLOR			:= \033[36m

build: $(FINAL)
	@$(EECHO) "${TITLE_COLOR}-----All build done-----\033[0m"

startup:
	@$(EECHO) "${TITLE_COLOR}-----Strat building-----\033[0m"

$(FINAL): startup $(LIBUSER)
	@$(EECHO) "${TITLE_COLOR}-----Generating link_app.asm-----\033[0m"
	@$(ROOT)/linker.sh
	@$(EECHO) "${TITLE_COLOR}-----Building os kernel-----\033[0m"
	@$(CD) $(KERNEL) && $(MAKE) -e build

$(LIBUSER): 
	@$(EECHO) "${TITLE_COLOR}-----Building user lib-----\033[0m"
	@$(CD) $(USER) && $(MAKE) -e build

.PHONY: clean
clean:
	@$(CD) $(USER) && $(CARGO) clean
	@$(CD) $(KERNEL) && $(CARGO) clean
	@if [ -f $(LINKER) ]; then $(RM) $(LINKER);fi