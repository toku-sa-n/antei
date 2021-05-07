ARCH	=	x86_64

BUILD_DIR	=	build

BOOTLOADER_DIR	=	bootloader
BOOTX64_SRC_DIR	=	$(BOOTLOADER_DIR)/bootx64
BOOTX64_SRCS	=	$(shell find $(BOOTLOADER) -name *.rs)
BOOTX64_SRCS 	+=	$(BOOTX64_SRC_DIR)/Cargo.toml
BOOTX64_SRCS	+=	$(BOOTX64_SRC_DIR)/.cargo/config.toml
BOOTX64_EXE	=	target/$(ARCH)-pc-windows-gnu/debug/bootx64.exe
BOOTX64	=	$(BUILD_DIR)/bootx64.efi

ISO_FILE	=	$(BUILD_DIR)/antei.iso

.PHONY:	all clean

all: $(ISO_FILE)

$(ISO_FILE): $(BOOTX64)|$(BUILD_DIR)
	dd if=/dev/zero of=$@ count=65536
	mformat -i $@ -h 200 -t 500 -s 144::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(BOOTX64) ::/efi/boot

$(BOOTX64): $(BOOTX64_EXE)|$(BUILD_DIR)
	mv $^ $@

$(BOOTX64_EXE): $(BOOTX64_SRCS)|$(BUILD_DIR)
	cd $(BOOTX64_SRC_DIR) && cargo build

$(BUILD_DIR):
	mkdir -p $@

clean:
	rm -rf $(BUILD_DIR)
