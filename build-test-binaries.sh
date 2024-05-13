/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -Wl,-Ttext=0x80000000 -nostdlib -o ./tests/add-addi.elf ./tests/add-addi.s
/opt/riscv/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./tests/add-addi.elf  ./tests/add.bin
#rm ./tests/add-addi
# /opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -S ./tests/app.c -o ./tests/app.s
/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -Bstatic -nodefaultlibs -nostdlib -Qn -O0 -Ttext=0x80000000 ./tests/app.c -o ./tests/app.elf
/opt/riscv/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./tests/app.elf ./tests/app.bin
# rm ./tests/app
