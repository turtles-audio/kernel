#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
