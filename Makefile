###############################################################################
# Project Info

CARGO_TARGET = riscv32imc-unknown-none-elf
CARGO_PROFILE = debug

###############################################################################


###############################################################################
# Directories

SRCDIR = src/
SRCS = $(wildcard $(SRCDIR)*.rs)
LIBS = $(wildcard lib/*.rs) $(wildcard lib/peripherals/*.rs)

BINDIR = bin/
BINARIES = $(addprefix $(BINDIR), $(notdir $(SRCS:.rs=.bin)))

PROJECTS = $(notdir $(SRCS:.rs=))

STARTUP = startup

###############################################################################


###############################################################################
# Toolchain

RV_PREFIX = 	/opt/rv32/bin/riscv32-unknown-elf-
OBJDUMP = 		$(RV_PREFIX)objdump
OBJCOPY = 		$(RV_PREFIX)objcopy

###############################################################################


###############################################################################
# Misc

RM      = rm -rf
MKDIR   = @mkdir -p $@
CP      = @cp $^ $@

###############################################################################


all: $(BINARIES)

$(BINDIR)%.bin: $(BINDIR)%.elf
	@echo "\n[ELF]	Creating bin file: $@"
	$(OBJCOPY) -O binary $^ $@

.PRECIOUS: $(BINDIR)%.elf
$(BINDIR)%.elf: target/$(CARGO_TARGET)/$(CARGO_PROFILE)/% | $(BINDIR)
	@echo "\n[ELF] Creating elf file: $@"
	$(CP)

$(BINDIR):
	$(MKDIR)

target/$(CARGO_TARGET)/$(CARGO_PROFILE)/%: $(SRCDIR)%.rs $(LIBS) src/$(STARTUP).s
	@echo "\n[ELF]	Creating cargo elf file: $@"
	@cargo +nightly build --bin $(notdir $@)

$(PROJECTS): %: $(BINDIR)%.bin

clean:
	@cargo clean
	-$(RM) bin

.PHONY: all clean

dump_%: $(BINDIR)%.elf
	$(OBJDUMP) -D $(BINDIR)$(subst dump_,,$@).elf > $(BINDIR)$@.txt