// LEDs are controller through GPIO
use super::gpio;

/// Specify the LED color
pub enum Color {
    Green, // Pin I12
    Orange, // Pin I13
    Red, // Pin I14
    Blue, // Pin I15
}

/// Turn on a specific LED
pub fn on(color: Color) {
    // LEDs are active low
    gpio::write(color.into(), gpio::State::Low).unwrap();
}

/// Turn off a specific LED
pub fn off(color: Color) {
    // LEDs are active low
    gpio::write(color.into(), gpio::State::High).unwrap();
}

/// Initialize GPIO MODERs and turn off LEDs
pub fn init() {
    // Configure MODERs
    gpio::toggle(gpio::Pin::I12, gpio::State::High).unwrap();
    gpio::toggle(gpio::Pin::I13, gpio::State::High).unwrap();
    gpio::toggle(gpio::Pin::I14, gpio::State::High).unwrap();
    gpio::toggle(gpio::Pin::I15, gpio::State::High).unwrap();

    // Turn off all LEDs (default is on)
    off(Color::Green);
    off(Color::Orange);
    off(Color::Red);
    off(Color::Blue);
}
