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

## 4. Meet Your Hardware

- A Microcontroller
- LEDs & LED matrix
- Two user buttons
- A reset button
- A USB port
- A sensor (magnetometer & accelerometer)

### Microcontroller

- Nordic nRF52833 chip
- Arm Cortex 32-bit processor
- Bluetooth Low Energy & 2.4 GHz SoC

### Rust Embedded Terminology

#### Peripheral Access Crate (PAC)

- Provides a safe(ish) direct interface to the peripherals of the chip
- Allows you to configure every last bit however you want
- e.g. [nRF52](https://crates.io/crates/nrf52833-pac)

#### Hardware Abstraction Layer (HAL)

- Builds on top of the chip's PAC
- Provides an abstraction for someone who does not know about all the special behavior of this chip
- Usually abstracts whole peripherals away into single structs
- e.g. [nRF52-hal](https://crates.io/crates/nrf52833-hal)

#### Board Support Crate (BSP)

- Abstract a whole board (e.g. micro:bit) away at once
- Provides abstractions to use both the microcontroller & the sensors, LEDs etc.
- Quite often (especially for custom-made boards) no pre-built BSP will be available
- e.g. [micro:bit BSP](https://crates.io/crates/microbit-v2)

#### `embedded-hal`

- Provides a set of traits that describe behavior which is usually shared across all implementations of a specific peripheral in all the HALs
- e.g. functions that are capable of turning the power on a pin either on or off: to switch an LED on and off on the board or whatever
- Allows us to write a driver for some piece of hardware that can be used on any chip for which an implementation of the `embedded-hal` traits exists
