#/usr/bin/make

# env
export PATH		:= /home/chipen/.cargo/bin/:$(PATH)

# path
OBJ				:= ${KERNEL}/target/riscv64gc-unknown-none-elf
SRC				:= ${KERNEL}/src

ifeq (${TYPE}, release)
    override TYPE := release
    override TYPE_DIR := release
else
    override TYPE := 
    override TYPE_DIR := debug
endif

# variables
LOG				:= info
TITLE_COLOR		:= \033[34m

# important files
ELF			:= ${OBJ}/${TYPE_DIR}/os
BIN			:= ${OBJ}/${TYPE_DIR}/os.bin

build: ${ELF}
	@$(EECHO) "${TITLE_COLOR}-----build done-----\033[0m"

run: ${BIN}
	@$(EECHO) "${TITLE_COLOR}-----starting qemu-----\033[0m"
	@$(QEMU) -machine virt -nographic \
	-bios ${KERNEL}/../bootloader/rustsbi-qemu.bin \
	-device loader,file=${BIN},addr=0x80200000

${ELF}: ${SRC}/main.rs
	@$(EECHO) "${TITLE_COLOR}-----building elf file-----\033[0m"
	@$(CARGO) build --${TYPE}

${BIN}: ${ELF}
	@$(EECHO) "${TITLE_COLOR}-----building bin file-----\033[0m"
	@$(OBJCOPY) --binary-architecture=riscv64 ${ELF} -O binary ${BIN}

.PHONY: clean
clean:
	@$(CARGO) clean