/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -Wl,-Ttext=0x80000000 -nostdlib -o ./test-files/add-addi.elf ./test-files/add-addi.s
/opt/riscv/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./test-files/add-addi.elf  ./test-files/add.bin
#rm ./test-files/add-addi
# /opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -S ./test-files/app.c -o ./test-files/app.s
/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -Bstatic -nodefaultlibs -nostdlib -Qn -O0 -Ttext=0x80000000 ./test-files/app.c -o ./test-files/app.elf
/opt/riscv/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./test-files/app.elf ./test-files/app.bin
# rm ./test-files/app
