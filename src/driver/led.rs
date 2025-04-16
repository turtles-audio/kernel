use super::gpio;

pub enum Color {
    Green,
    Orange,
    Red,
    Blue,
}

pub fn on(color: Color) {
    gpio::write(color.into(), gpio::State::Low);
}

pub fn off(color: Color) {
    gpio::write(color.into(), gpio::State::High);
}

pub fn init() {
    gpio::toggle(gpio::Pin::I12, gpio::State::High);
    gpio::toggle(gpio::Pin::I13, gpio::State::High);
    gpio::toggle(gpio::Pin::I14, gpio::State::High);
    gpio::toggle(gpio::Pin::I15, gpio::State::High);

    off(Color::Green);
    off(Color::Orange);
    off(Color::Red);
    off(Color::Blue);
}
