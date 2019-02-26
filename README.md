It needs riscv64-unknown-none-elf toolchian installed


Compile with:
```cargo build --release --target=riscv64gc-unknown-none-elf --verbose```

Convert to bin:
```riscv64-unknown-elf-objcopy -O binary target/riscv64gc-unknown-none-elf/release/hello_world hello_world.bin```

Flash with: kflash.py
