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

```sh
rustup component add llvm-tools
cargo install cargo-binutils --vers '^0.3'
cargo size --version
```

### `probe-rs-tools`

```sh
brew install probe-rs/probe-rs/probe-rs
cargo embed --version
```

### `gdb`

```sh
brew install gdb
```

### `minicom`

```sh
brew install minicom
```

### `lsusb`

```sh
brew install lsusb
```

### Verify the Installation

```sh
probe-rs list
probe-rs info
```

```sh
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

## 5. Meet Your Software

### Embedded Setup

```rust
// main.rs
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit as _;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {
        asm::nop();
    }
}
```

```toml
# .cargo/config.toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52833_xxAA"
rustflags = [
  "-C", "linker=rust-lld",
]
```

```toml
# Embed.toml
[default.general]
chip = "nrf52833_xxAA"

[default.reset]
halt_afterwards = true

[default.rtt]
enabled = false

[default.gdb]
enabled = true
```

### Build It

#### Targets

- `thumbv6m-none-eabi`, for the Cortex-M0 and Cortex-M1 processors
- `thumbv7m-none-eabi`, for the Cortex-M3 processor
- `thumbv7em-none-eabi`, for the Cortex-M4 and Cortex-M7 processors
- `thumbv7em-none-eabihf`, for the Cortex-M4F and Cortex-M7F processors
- `thumbv8m.main-none-eabi`, for the Cortex-M33 and Cortex-M35P processors
- `thumbv8m.main-none-eabihf`, for the Cortex-M33F and Cortex-M35PF processors

#### Pre-compiled Standard Library

```sh
rustup target add thumbv7em-none-eabihf
```

#### Cross-compile Program

```sh
# cd source/mdbook/05-meet-your-software
cargo build --example init
```

#### Verify Binary

```sh
cargo readobj --example init -- --file-headers
```

### Flash It

```sh
cargo embed --example init
```

### Debug It

```sh
gdb ../../../target/thumbv7em-none-eabihf/debug/examples/init
```

#### Connecting

```sh
target remote :1337
```

#### Breakpoints

```sh
break main
break <line-num>
```

```sh
continue
```

```sh
info break
```

```sh
delete <breakpoint-num>
```

#### TUI

```sh
layout src
layout asm
```

```sh
tui disable
```

#### Printing

```sh
info locals
```

```sh
print x
print &x
```

#### Execute Next Line

```sh
next
```

#### Execute Next Instruction

```sh
stepi
```

#### Back To Main

```sh
monitor reset
```

#### Quit

```sh
quit
```

## 6. Hello World

```sh
cd discovery-mb2/source/mdbook/src/06-hello-world
```

```sh
cargo run --example light-up
```

```rust
// examples/light-up.rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_hal::{gpio, pac};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let _row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    #[allow(clippy::empty_loop)]
    loop {}
}
```

### Timers

```rust
// examples/timers.rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use nrf52833_hal::{gpio, pac, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    let mut timer0 = timer::Timer::new(peripherals.TIMER0);

    loop {
        timer0.delay_ms(500);
        row1.set_high().unwrap();
        timer0.delay_ms(500);
        row1.set_low().unwrap();
    }
}
```

### Board Support Crate

```rust
// main.rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::hal::{gpio, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let mut row1 = board
        .display_pins
        .row1
        .into_push_pull_output(gpio::Level::High);
    let _col1 = board
        .display_pins
        .col1
        .into_push_pull_output(gpio::Level::Low);

    let mut timer0 = timer::Timer::new(board.TIMER0);

    loop {
        timer0.delay_ms(500);
        row1.set_high().unwrap();
        timer0.delay_ms(500);
        row1.set_low().unwrap();
    }
}
```

## 7. LED Roulette

```rust
// examples/light-it-all.rs
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let light_it_all = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];

    loop {
        display.show(&mut timer, light_it_all, 1000);
        display.clear();
        timer.delay_ms(1000);
    }
}
```
