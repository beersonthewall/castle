.PHONY: all kernel clean check bootloader

all: bootloader kernel
	@mkdir -p target/esp/efi/boot/
	@cp target/x86_64-unknown-uefi/debug/bootloader.efi \
	target/esp/efi/boot/bootx64.efi
	@qemu-system-x86_64 -enable-kvm \
	-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
	-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
	-drive format=raw,file=fat:rw:target/esp

bootloader:
	@cargo b -p bootloader --target x86_64-unknown-uefi

kernel:
	@cargo b -p kernel --target x86_64-unknown-none

check:
	@cargo check

clean: 
	@cargo clean
	@rm -rf target/

