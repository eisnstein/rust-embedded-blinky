#![no_main]
#![no_std]

extern crate panic_halt;

use core::{cell::RefCell, ops::Deref};

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use cortex_m_semihosting::{self, hprintln};
use stm32f3::stm32f303::{self, interrupt, EXTI, GPIOE};

static EXT_I: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));
static GPIO_E: Mutex<RefCell<Option<GPIOE>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // You should see that in your openocd output
    hprintln!("Hello from Discovery");

    let peripherals = stm32f303::Peripherals::take().unwrap();

    let syscfg = &peripherals.SYSCFG;
    let exti = &peripherals.EXTI;

    // Initialize EXT interrupt

    // Set Pin 0 from Port A as input for EXTI0
    syscfg.exticr1.write(|w| w.exti0().pa0());
    // Disable mask on EXTI0
    exti.imr1.write(|w| w.mr0().set_bit());
    // Set rising trigger disabled
    exti.rtsr1.write(|w| w.tr0().disabled());
    // Set falling trigger enabled
    exti.ftsr1.write(|w| w.tr0().enabled());
    // Enable interrupt
    unsafe {
        NVIC::unmask(stm32f303::Interrupt::EXTI0);
    }

    let rcc = &peripherals.RCC;
    // Set HSI clock on
    rcc.cr.write(|w| w.hsion().set_bit());
    // Enable GPIO Port E and A clock
    rcc.ahbenr.write(|w| w.iopaen().enabled());
    rcc.ahbenr.write(|w| w.iopeen().enabled());
    // Enable SYSCFG clock
    rcc.apb2enr.write(|w| w.syscfgen().enabled());

    let gpioa = &peripherals.GPIOA;
    // Set Pin 0 to input
    gpioa.moder.write(|w| w.moder0().input());

    let gpioe = &peripherals.GPIOE;
    // Set Pin 9 to output
    gpioe.moder.write(|w| w.moder9().output());

    // Put exti and gpioe into the global variable to be able to
    // access Port E in the EXTI0 interrupt handler.
    cortex_m::interrupt::free(|cs| {
        EXT_I.borrow(cs).replace(Some(peripherals.EXTI));
        GPIO_E.borrow(cs).replace(Some(peripherals.GPIOE));
    });

    loop {}
}

#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(exti) = EXT_I.borrow(cs).borrow().deref() {
            // Clear pending register
            exti.pr1.write(|w| w.pr0().set_bit());
        }

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
