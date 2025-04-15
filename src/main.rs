#![no_main]
#![no_std]

mod driver;
mod panic;
mod runtime;

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    loop {}
}
