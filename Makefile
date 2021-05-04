ARCH	=	x86_64

BUILD_DIR	=	build

BOOTX64_SRC_DIR	=	bootx64
BOOTX64_SRCS	=	$(shell find $(BOOTX64_SRC_DIR) -name *.rs)
BOOTX64_SRCS 	+=	$(BOOTX64_SRC_DIR)/Cargo.toml
BOOTX64_SRCS	+=	$(BOOTX64_SRC_DIR)/.cargo/config.toml
BOOTX64_DLL	=	$(BOOTX64_SRC_DIR)/target/$(ARCH)-pc-windows-gnu/debug/bootx64.dll
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

$(BOOTX64): $(BOOTX64_DLL)|$(BUILD_DIR)
	objcopy --target=efi-app-$(ARCH) $^ $@

$(BOOTX64_DLL): $(BOOTX64_SRCS)|$(BUILD_DIR)
	cd $(BOOTX64_SRC_DIR) && cargo build

$(BUILD_DIR):
	mkdir -p $@

clean:
	rm -rf $(BUILD_DIR)
