#![no_main]
#![no_std]

extern crate panic_halt;

use core::{cell::RefCell, ops::Deref};

use cortex_m::{interrupt::Mutex, peripheral::syst::SystClkSource};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{self, hprintln};
use stm32f3::stm32f303::{self, GPIOE, SYST};

const HSI_CLOCK: u32 = 8_000_000;

static GPIO_E: Mutex<RefCell<Option<GPIOE>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // You should see that in your openocd output
    hprintln!("Hello from Discovery");

    let peripherals = stm32f303::Peripherals::take().unwrap();
    let mut systick = stm32f303::CorePeripherals::take().unwrap().SYST;

    init_systick(&mut systick, 1_000);

    let rcc = &peripherals.RCC;
    // Set HSI clock on
    rcc.cr.write(|w| w.hsion().set_bit());
    // Enable GPIO Port E clock
    rcc.ahbenr.write(|w| w.iopeen().enabled());
    // Enable SYSCFG clock
    rcc.apb2enr.write(|w| w.syscfgen().enabled());

    let gpioe = peripherals.GPIOE;
    // Set Pin 9 to output
    gpioe.moder.write(|w| w.moder9().output());

    // Put gpioe into the global variable to be able to
    // access Port E in the SysTick interrupt handler.
    cortex_m::interrupt::free(|cs| GPIO_E.borrow(cs).replace(Some(gpioe)));

    loop {}
}

fn init_systick(systick: &mut SYST, ms: u32) {
    let seconds = (ms / 1_000) as f32;
    let count_val = (HSI_CLOCK as f32 * seconds) as u32 - 1;

    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(count_val);
    systick.clear_current();
    systick.enable_counter();
    systick.enable_interrupt();
}

#[exception]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        if let Some(gpioe) = GPIO_E.borrow(cs).borrow().deref() {
            // Read Port E Pin 9 output data register
            if gpioe.odr.read().odr9().bit_is_set() {
                // Write to bit reset register to set Pin 9 Low
                gpioe.bsrr.write(|w| w.br9().set_bit());
            } else {
                // Write to bit set register to set Pin 9 High
                gpioe.bsrr.write(|w| w.bs9().set_bit());
            }
        }
    })
}
