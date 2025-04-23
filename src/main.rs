#![no_main]
#![no_std]

mod driver;
mod video;
mod panic;
mod runtime;

use driver::*;
use video::frame::{Color, Layer, Side};

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    rcc::peripherals();
    led::init();

    led::on(led::Color::Green);

    loop {}
}

#[unsafe(no_mangle)]
pub fn fault() -> ! {
    led::init();

    led::on(led::Color::Red);

    loop {}
}