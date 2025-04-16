#![no_main]
#![no_std]

mod driver;
mod panic;
mod runtime;

use driver::*;

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    rcc::peripherals();
    led::init();

    led::on(led::Color::Blue);

    loop {}
}
