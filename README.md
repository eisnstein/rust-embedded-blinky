# Rust Embedded Blinky

This project implements the "Hello World" of embedded programming in Rust in different ways. From a more or less _raw_ approach to using the [stm32f3](https://docs.rs/stm32f3/0.15.1/stm32f3/) hal crate. The different stages are tagged.

Order is:

- [raw-naive-delay](https://github.com/eisnstein/rust-embedded-blinky/tree/raw-naive-delay): Unsafe Rust with a naive delay function.
- [hal-naive-delay](https://github.com/eisnstein/rust-embedded-blinky/tree/hal-naive-delay): STM32F3 hal crate with a naive delay function.
- [hal-manual-delay](https://github.com/eisnstein/rust-embedded-blinky/tree/hal-manual-delay): STM32F3 hal crate with manual on/off via a button.
- [hal-systick-delay](https://github.com/eisnstein/rust-embedded-blinky/tree/hal-systick-delay): STM32F3 hal crate with SysTick interrupt.
