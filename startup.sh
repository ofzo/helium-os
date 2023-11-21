export PATH="/usr/local/Cellar/qemu/8.1.1/bin:$PATH"

cd std && make build && cd ..
if [[ $? -ne 0 ]]; then
    exit $?
fi

cd os && make build && cd ..
if [[ $? -ne 0 ]]; then
    exit $?
fi

target=os/target/riscv64gc-unknown-none-elf/release/helium.os

qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./bootloader/rustsbi-qemu.bin \
    -device loader,file=$target,addr=0x80200000 \
    # -s -S

# ctrl-a x exit qemu
