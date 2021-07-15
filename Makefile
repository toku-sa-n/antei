ARCH	=	x86_64

ifeq ($(RELEASE), 1)
	RELEASE_OR_DEBUG	=	release
	RUSTFLAGS	=	--release
else
	RELEASE_OR_DEBUG	=	debug
endif

BUILD_DIR	=	build/$(RELEASE_OR_DEBUG)

BOOTLOADER_DIR	=	bootloader
BOOTX64_SRC_DIR	=	$(BOOTLOADER_DIR)/bootx64
BOOTX64_SRCS	=	$(shell find $(BOOTLOADER) -name *.rs)
BOOTX64_SRCS 	+=	$(BOOTX64_SRC_DIR)/Cargo.toml
BOOTX64_SRCS	+=	$(BOOTX64_SRC_DIR)/.cargo/config.toml
BOOTX64_EXE	=	target/$(ARCH)-pc-windows-gnu/$(RELEASE_OR_DEBUG)/bootx64.exe
BOOTX64	=	$(BUILD_DIR)/bootx64.efi

KERNEL_DIR	=	kernel
KERNEL_SRCS	=	$(shell find $(KERNEL_DIR) -name *.rs)
KERNEL_SRCS	+=	$(KERNEL_DIR)/Cargo.toml
KERNEL_SRCS	+=	$(KERNEL_DIR)/.cargo/config.toml
KERNEL_SRCS	+=	$(KERNEL_DIR)/kernel.ld
KERNEL	=	target/$(ARCH)-unknown-linux-gnu/$(RELEASE_OR_DEBUG)/kernel

ISO_FILE	=	$(BUILD_DIR)/antei.iso

QEMU	=	qemu-system-x86_64
QEMU_PARAMS	=	-drive if=pflash,format=raw,file=OVMF_CODE.fd,readonly=on	\
				-drive if=pflash,format=raw,file=OVMF_VARS.fd,readonly=on	\
				-drive format=raw,file=$(ISO_FILE)	\
				-m 4G	\
				-serial stdio	\
				-display none

.PHONY:	all run clean

all: $(ISO_FILE)

$(ISO_FILE): $(KERNEL) $(BOOTX64)|$(BUILD_DIR)
	dd if=/dev/zero of=$@ count=65536
	mformat -i $@ -h 200 -t 500 -s 144::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(KERNEL) ::/
	mcopy -i $@ $(BOOTX64) ::/efi/boot

$(KERNEL): $(KERNEL_SRCS)
	cd $(KERNEL_DIR) && cargo build $(RUSTFLAGS)

$(BOOTX64): $(BOOTX64_EXE)|$(BUILD_DIR)
	cp $^ $@

$(BOOTX64_EXE): $(BOOTX64_SRCS)
	cd $(BOOTX64_SRC_DIR) && cargo build $(RUSTFLAGS)

$(BUILD_DIR):
	mkdir -p $@

run: $(ISO_FILE)
	$(QEMU) $(QEMU_PARAMS)

clean:
	rm -rf $(BUILD_DIR)
	cargo clean
