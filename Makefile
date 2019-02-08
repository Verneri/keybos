SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) link.ld

.PHONY: all clean

all: clean kernel8.img

target/aarch64-unknown-none/release/keybos: $(SOURCES)
	cargo xbuild --target=aarch64-unknown-none --release

kernel8.img: target/aarch64-unknown-none/release/keybos
	cargo objcopy -- --strip-all -O binary $< kernel8.img


clean:
	-rm kernel8.img
	cargo clean
