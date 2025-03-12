# OS from Scratch in Rust

![Running the OS on a QEMU system emulator](./public/intro.png "Working")
To run the project, you need to have cargo installed
on your system. You can download it from https://rustup.rs/

## Building the project

Run `cargo build` from the root or `cargo build --release`
for more optimized build.

You need to have QEMU system emulator which can be installed
from here https://www.qemu.org/download/

## Running the project

The project can be run using the following commands 
```
# To run the BIOS image
cargo run --bin qemu-bios

# To run the UEFI image
cargo run --bin qemu-uefi
```

## Rendering text and color in UEFI
![Rendering of text](./public/demo_1.png "Text")
![Showing colored frame inside the OS](./public/demo_2.png "Shape")