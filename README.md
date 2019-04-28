This repo is now archived see:

https://github.com/riscv-rust/k210-example for sample of rust code on k210,

https://github.com/riscv-rust/k210-pac for k210 peripheral access crate made by rust-riscv community.



This project was created as test to run rust code on k210 chip.

It needs riscv64-unknown-none-elf toolchian installed

Compile with:
```cargo build --release --target=riscv64gc-unknown-none-elf --verbose```

Convert to bin:
```riscv64-unknown-elf-objcopy -O binary target/riscv64gc-unknown-none-elf/release/hello_world hello_world.bin```

Flash with: kflash.py
