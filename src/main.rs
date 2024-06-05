#![no_main]
#![no_std]

extern crate panic_halt;

use core::arch::asm;

use cortex_m_rt::entry;
use cortex_m_semihosting::{self, hprintln};
use stm32f3::stm32f303::{self};

#[entry]
fn main() -> ! {
    // You should see that in your openocd output
    hprintln!("Hello from Discovery");

    let peripherals = stm32f303::Peripherals::take().unwrap();

    let rcc = &peripherals.RCC;
    // Set HSI clock on
    rcc.cr.write(|w| w.hsion().set_bit());
    // Enable GPIO Port E clock
    rcc.ahbenr.write(|w| w.iopeen().set_bit());

    let gpioe = &peripherals.GPIOE;

    // Set Pin 9 to output
    gpioe.moder.write(|w| w.moder9().output());

    loop {
        // Read Port E Pin 9 output data register
        if gpioe.odr.read().odr9().bit_is_set() {
            // Write to bit reset register to set Pin 9 Low
            gpioe.bsrr.write(|w| w.br9().set_bit());
        } else {
            // Write to bit set register to set Pin 9 High
            gpioe.bsrr.write(|w| w.bs9().set_bit());
        }

        unsafe {
            delay();
        }
    }
}

unsafe fn delay() {
    // Delay for approx 1s - found 80.000 by "brute force"
    for _ in 0..80_000 {
        asm!("nop");
    }
}
