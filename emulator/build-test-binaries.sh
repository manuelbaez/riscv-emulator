/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -Wl,-Ttext=0x0 -nostdlib -o ./test-files/add-addi ./test-files/add-addi.s
/opt/riscv/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./test-files/add-addi  ./test-files/add.bin
rm ./test-files/add-addi
/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -S ./test-files/app.c -o ./test-files/app.s
/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc -Bstatic -nodefaultlibs -nostdlib -Qn -Ttext=0x0 ./test-files/app.c -o ./test-files/app
/opt/riscv/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./test-files/app  ./test-files/app.bin
# rm ./test-files/app
