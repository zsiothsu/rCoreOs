#/usr/bin/make

# path
OBJ				= $(USER)/target/riscv64gc-unknown-none-elf
SRC				= $(USER)/src

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
PROJ		:= user_lib
SRC_FILES	:= $(wildcard $(SRC)/bin/*.rs)
ELF_FILES	:= $(patsubst $(SRC)/bin/%.rs, $(OBJ)/$(TYPE_DIR)/%, $(SRC_FILES))
BIN_FILES	:= $(patsubst $(SRC)/bin/%.rs, $(OBJ)/$(TYPE_DIR)/%.bin, $(SRC_FILES))
MAIN_FILR	:= lib.rs

build: ${BIN_FILES}
	@echo $(SRC)
	@$(EECHO) "${TITLE_COLOR}-----build done-----\033[0m"

${ELF_FILES}: ${SRC}/${MAIN_FILR}
	@$(EECHO) "${TITLE_COLOR}-----building elf file-----\033[0m"
	@$(CARGO) build --${TYPE}

${BIN_FILES}: $(OBJ)/$(TYPE_DIR)/%.bin : $(OBJ)/$(TYPE_DIR)/%
	@$(EECHO) "${TITLE_COLOR}-----building bin file $<-----\033[0m"
	@$(OBJCOPY) --binary-architecture=riscv64 $< -O binary ${BIN} $@

.PHONY: clean
clean:
	@$(CARGO) clean