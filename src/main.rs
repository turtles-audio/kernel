#![no_main]
#![no_std]

mod driver;
mod video;
mod panic;
mod runtime;

use driver::{ltdc::Porch, *};
use video::frame::{Color, Layer, Side};

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    rcc::peripherals();
    led::init();

    led::on(led::Color::Green);
    gpio::toggle(gpio::Pin::G3, gpio::State::High).unwrap();
    gpio::pull(gpio::Pin::G3, gpio::Pull::Up);
    gpio::speed(gpio::Pin::G3, gpio::Speed::VeryHigh);
    gpio::write(gpio::Pin::G3, gpio::State::Low).unwrap();

    for i in 0..10000000 {
        core::hint::black_box(i);
    }

    dsi::wrapper(false);

    gpio::write(gpio::Pin::G3, gpio::State::High).unwrap();
    led::on(led::Color::Blue);

    ltdc::init();

    gpio::toggle(gpio::Pin::J2, gpio::State::High).unwrap();
    gpio::write(gpio::Pin::J2, gpio::State::High).unwrap();

    ltdc::timing(Porch::new(1, 1), Porch::new(1, 1), Porch::new(480, 800), Porch::new(1, 1));
    ltdc::layer(&Layer::new(150, 150, Side::Top));
    dsi::ltdc(true);
    //dsi::wrapper(true);

    loop {}
}

#[unsafe(no_mangle)]
pub fn fault() -> ! {
    led::init();

    led::on(led::Color::Red);

    loop {}
}