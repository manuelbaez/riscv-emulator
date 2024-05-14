build-kernel:
    ./tests/linux-kernel/build.sh
run-linux:
    cargo run --features debug --release ./tests/linux-kernel/kernel.bin
build-test-binaries:
    ./build-test-binaries.sh
run-app-test:
    just build-test-binaries && cargo run --features debug --release ./tests/app.bin
