/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -Wl,-Ttext=0x0 -nostdlib -o ./test-files/add-addi ./test-files/add-addi.s
/opt/riscv/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./test-files/add-addi  ./test-files/add.bin
rm ./test-files/add-addi