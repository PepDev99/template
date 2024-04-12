###############################################################################
# Project Info

PROJECT = main
MCU = DEMOSYSTEM

CARGO_TARGET = riscv32imc-unknown-none-elf
CARGO_PROFILE = debug

###############################################################################

###############################################################################
# Directories

SRCS = 		$(wildcard src/*.rs) $(wildcard lib/*.rs)
STARTUP = 	startup

###############################################################################
# Toolchain

RV_PREFIX = 	/opt/rv32/bin/riscv32-unknown-elf-
OBJDUMP = 		$(RV_PREFIX)objdump
OBJCOPY = 		$(RV_PREFIX)objcopy

###############################################################################

###############################################################################
# Misc

RM      = rm -rf 					# Remove recursively command
MKDIR   = @mkdir -p $(@D) 			# Creates folders if not present
CP      = @cp target/$(CARGO_TARGET)/$(CARGO_PROFILE)/$(PROJECT) $@

###############################################################################

all: bin/$(PROJECT).bin

bin/$(PROJECT).elf: $(SRCS) src/$(STARTUP).s
	@echo "\n[ELF]	Creating elf file"
	$(MKDIR)
	@cargo +nightly build --bin $(PROJECT)
	$(CP)

bin/$(PROJECT).bin: bin/$(PROJECT).elf
	@echo "\n[ELF]	Creating bin file"
	$(OBJCOPY) -O binary bin/$(PROJECT).elf bin/$(PROJECT).bin

clean:
	@cargo clean
	-$(RM) bin

.PHONY: all clean

dump:
	$(OBJDUMP) -D bin/$(PROJECT).elf


