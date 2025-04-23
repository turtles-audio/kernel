TARGET=target
TOOLCHAIN=thumbv7em-none-eabihf
MODE=release

BINARY=kernel
OUTPUT=$(TARGET)\$(TOOLCHAIN)\$(MODE)\$(BINARY)

all: $(OUTPUT)
	cp "$<" "$<.elf"

$(OUTPUT): .FORCE
	cargo build --release

.PHONY: .FORCE