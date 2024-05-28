#![no_main]
#![no_std]

extern crate panic_halt;

use core::{
    arch::asm,
    ptr::{self},
};

use cortex_m_rt::entry;
use cortex_m_semihosting::{self, hprintln};

const RCC: u32 = 0x4002_1000;
const RCC_CR: *mut u32 = (RCC + 0x00) as *mut u32;
const RCC_AHBENR: *mut u32 = (RCC + 0x14) as *mut u32;

const GPIOE: u32 = 0x4800_1000;
const GPIOE_MODER: *mut u32 = (GPIOE + 0x00) as *mut u32;
const GPIOE_ODR: *mut u32 = (GPIOE + 0x14) as *mut u32;
const GPIOE_BSRR: *mut u32 = (GPIOE + 0x18) as *mut u32;

#[entry]
fn main() -> ! {
    // You should see that in your openocd output
    hprintln!("Hello from Discovery");

    unsafe {
        let mut val = ptr::read_volatile(RCC_CR);
        // Use the HSI as system clock (8MHz)
        val |= 1 << 1;
        ptr::write_volatile(RCC_CR, val);

        val = ptr::read_volatile(RCC_AHBENR);
        // Enable GPIO E clock
        val |= 1 << 21;
        ptr::write_volatile(RCC_AHBENR, val);

        // Set Pin 9 to output: PE 9 = Bit 19 & 18
        val = ptr::read_volatile(GPIOE_MODER);
        val &= 0xFFF3_FFFF; // Clear Bit 19 & 18
        val |= 1 << 18; // Set Bit 18
        ptr::write_volatile(GPIOE_MODER, val);

        loop {
            // Read in the output data register of GPIO Port E
            let val = ptr::read_volatile(GPIOE_ODR);
            // If Pin 9 is currently in High state, switch to Low, otherwise switch to High
            if val & (1 << 9) == (1 << 9) {
                // Set Pin 18 Low
                ptr::write_volatile(GPIOE_BSRR, 1 << 25);
            } else {
                // Set Pin 18 High
                ptr::write_volatile(GPIOE_BSRR, 1 << 9);
            }

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
