# micro:bit v2 Embedded Discovery Book

## 1. Background

### Microcontroller

> A microcontroller is a system on a chip. Whereas your computer is made up of several discrete components: a processor, RAM, storage, an Ethernet port, etc.; a microcontroller has all those types of components built into a single "chip" or package. This makes it possible to build systems with fewer parts.

### Advantages of Microcontrollers

- Cost
- Power consumption
- Responsiveness
- Reliability

## 2. Hardware Requirements

- micro:bit v2 board
- micro-B USB cable

## 3. Setting Up A Development Environment

### `rustc`

```shell
rustc -V # >=1.17.0
```

### `cargo-binutils`

```shell
rustup component add llvm-tools
cargo install cargo-binutils --vers '^0.3'
cargo size --version
```

### `probe-rs-tools`

```shell
brew install probe-rs/probe-rs/probe-rs
cargo embed --version
```

### `gdb`

```shell
brew install gdb
```

### `minicom`

```shell
brew install minicom
```

### `lsusb`

```shell
brew install lsusb
```

### Verify the Installation

```shell
probe-rs list
probe-rs info
```

```shell
cd source/mdbook/src/03-setup
rustup target add thumbv7em-none-eabihf
cargo embed --target thumbv7em-none-eabihf
```
