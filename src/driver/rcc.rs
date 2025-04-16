const RCC_BASE: u32 = 0x58024400;
const RCC_APB1HENR: u32 = RCC_BASE + 0xEC;
const RCC_APB1LENR: u32 = RCC_BASE + 0xE8;
const RCC_APB2ENR: u32 = RCC_BASE + 0xF0;
const RCC_APB3ENR: u32 = RCC_BASE + 0xE4;
const RCC_APB4ENR: u32 = RCC_BASE + 0xF4;

const RCC_AHB1ENR: u32 = RCC_BASE + 0xD8;
const RCC_AHB2ENR: u32 = RCC_BASE + 0xDC;
const RCC_AHB3ENR: u32 = RCC_BASE + 0xD4;
const RCC_AHB4ENR: u32 = RCC_BASE + 0xE0;

pub fn ltdc() -> Result<(), ()> {
    let reg: *mut u32 = RCC_APB3ENR as *mut u32;

    unsafe {
        *reg |= (1 << 3);
    }

    return Ok(());
}

pub fn gpio() -> Result<(), ()> {
    let reg: *mut u32 = RCC_AHB4ENR as *mut u32;

    unsafe {
        *reg |= 0b11111111111;
    }

    return Ok(());
}

pub fn peripherals() {
    ltdc();
    gpio();
}
