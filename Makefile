ARCH	=	x86_64

define cargo_project_src
	$(shell find $1|grep -v $1/target)
endef

ifeq ($(RELEASE), 1)
	RELEASE_OR_DEBUG	=	release
	RUSTFLAGS	=	--release
else
	RELEASE_OR_DEBUG	=	debug
endif

ifeq ($(MAKECMDGOALS), test)
	BUILD_DIR	=	build/$(RELEASE_OR_DEBUG)/test
else
	BUILD_DIR	=	build/$(RELEASE_OR_DEBUG)
endif

BOOTX64_DIR	=	bootx64
BOOTX64_SRCS	=	$(call cargo_project_src, $(BOOTX64_DIR))
BOOTX64_IN_TARGET	=	target/$(ARCH)-pc-windows-gnu/$(RELEASE_OR_DEBUG)/bootx64.exe
BOOTX64	=	$(BUILD_DIR)/bootx64.efi

KERNEL_DIR	=	kernel
KERNEL_SRCS	=	$(call cargo_project_src, $(KERNEL_DIR))
KERNEL_IN_TARGET	=	target/$(ARCH)-unknown-linux-gnu/$(RELEASE_OR_DEBUG)/kernel
KERNEL	=	$(BUILD_DIR)/kernel

INIT_DIR	=	servers/init
INIT_SRCS	=	$(call cargo_project_src, $(INIT_DIR))
INIT_IN_TARGET	=	target/$(ARCH)-unknown-linux-gnu/$(RELEASE_OR_DEBUG)/init
INIT	=	$(BUILD_DIR)/init

PM_DIR	=	servers/pm
PM_SRCS	=	$(call cargo_project_src, $(PM_DIR))
PM_IN_TARGET	=	target/$(ARCH)-unknown-linux-gnu/$(RELEASE_OR_DEBUG)/pm
PM	=	$(BUILD_DIR)/pm

INITRD_CONTENTS	=	init pm
INITRD	=	$(BUILD_DIR)/initrd.cpio

LIBS_DIR	=	libs
LIBS_SRCS	=	$(shell find $(LIBS_DIR)/)

ISO_FILE	=	$(BUILD_DIR)/antei.iso

QEMU	=	qemu-system-x86_64
QEMU_PARAMS	=	-drive if=pflash,format=raw,file=OVMF_CODE.fd,readonly=on	\
				-drive if=pflash,format=raw,file=OVMF_VARS.fd,readonly=on	\
				-drive format=raw,file=$(ISO_FILE)	\
				-m 4G	\
				-serial stdio	\
				-display none

.PHONY:	all run test clean

all: $(ISO_FILE)

$(ISO_FILE): $(KERNEL) $(INITRD) $(BOOTX64)|$(BUILD_DIR)
	dd if=/dev/zero of=$@ count=65536
	mformat -i $@ -h 200 -t 500 -s 144::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(KERNEL) ::/
	mcopy -i $@ $(INITRD) ::/
	mcopy -i $@ $(BOOTX64) ::/efi/boot

# Do not add a target like $(KERNEL_IN_TARGET).
# Otherwise `make test` may use the normal kernel binary, for example.
$(KERNEL): $(KERNEL_SRCS) $(LIBS_SRCS)|$(BUILD_DIR)
	(cd $(KERNEL_DIR) && cargo build $(RUSTFLAGS))
	cp $(KERNEL_IN_TARGET) $@

# Do not add a target like $(BOOTX64_IN_TARGET).
# Otherwise `make test` may use the normal $(BOOTX64_IN_TARGET) file, for example.
$(BOOTX64): $(BOOTX64_SRCS) $(LIBS_SRCS)|$(BUILD_DIR)
	(cd $(BOOTX64_DIR) && cargo build $(RUSTFLAGS))
	cp $(BOOTX64_IN_TARGET) $@

$(INITRD): $(INIT) $(PM)|$(BUILD_DIR)
	cd $(BUILD_DIR) && echo $(INITRD_CONTENTS)|tr " " "\n"|cpio -o > $(notdir $@)

$(INIT): $(INIT_SRCS) $(LIBS_SRCS)|$(BUILD_DIR)
	(cd $(INIT_DIR) && cargo build $(RUSTFLAGS))
	cp $(INIT_IN_TARGET) $@

$(PM): $(PM_SRCS) $(LIBS_SRCS)|$(BUILD_DIR)
	(cd $(PM_DIR) && cargo build $(RUSTFLAGS))
	cp $(PM_IN_TARGET) $@

$(BUILD_DIR):
	mkdir -p $@

run: $(ISO_FILE)
	$(QEMU) $(QEMU_PARAMS)

test: QEMU_PARAMS	+=	\
	-device isa-debug-exit,iobase=0xf4,iosize=0x04	\
	-no-reboot
test: RUSTFLAGS	+=	--features test_on_qemu
test: SUCCESS	=	33
test: $(ISO_FILE)
	cargo test $(RUSTFLAGS)
	$(QEMU) $(QEMU_PARAMS);\
	if [ $$? -eq $(SUCCESS) ];\
	then\
		echo Test succeeds!;\
	else\
		echo Test failed!;\
		exit 1;\
	fi

clean:
	rm -rf build
	cargo clean
