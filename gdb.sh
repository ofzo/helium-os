target=target/riscv64gc-unknown-none-elf/release/helium-os

# brew install riscv64-elf-gdb
riscv64-elf-gdb \
    -ex "file $target.os" \
    -ex "set arch riscv:rv64" \
    -ex 'target remote localhost:1234'
