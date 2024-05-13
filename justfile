build-kernel:
    ./linux-kernel-build/build.sh
run-linux: 
    cargo run --features debug --release ./tests/linux-kernel/kernel.bin