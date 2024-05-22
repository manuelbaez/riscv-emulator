export TOOLCHAIN_BIN_PATH=/opt/riscv
export KERNEL_PATH=/mnt/data/repos/linux-stable
export SOURCE_PATH="$(
    cd -- "$(dirname "$0")" >/dev/null 2>&1
    pwd -P
)"

cd ${KERNEL_PATH}

make mrproper
make distclean

cp ${SOURCE_PATH}/def.config ./.config
# CONFIG_RISCV_ISA_C=n
# CONFIG_ARCH_RV64I=y
# CFLAGS="-O3"
make ARCH=riscv \
    CROSS_COMPILE=${TOOLCHAIN_BIN_PATH}/bin/riscv64-unknown-linux-gnu- \
    -j$(nproc) </dev/null

${TOOLCHAIN_BIN_PATH}/bin/riscv64-unknown-linux-gnu-objcopy -O binary ./arch/riscv/boot/Image ${SOURCE_PATH}/kernel.bin
${TOOLCHAIN_BIN_PATH}/riscv64-unknown-linux-gnu/bin/objdump -D ${KERNEL_PATH}/arch/riscv/boot/Image >${SOURCE_PATH}/kernel-dump.txt

# cp ./arch/riscv/boot/xipImage ${SOURCE_PATH}/kernel.bin
# ${TOOLCHAIN_BIN_PATH}/riscv64-unknown-linux-gnu/bin/objdump -D ${KERNEL_PATH}/arch/riscv/boot/Image >${SOURCE_PATH}/kernel-dump.txt
# vmlinux
