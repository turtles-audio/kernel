const DSI_BASE: u32 = 0x50000000;

const DSI_WCR: u32 = DSI_BASE + 0x404;

pub fn wrapper(enable: bool) {
    let wcr = DSI_WCR as *mut u32;

    unsafe {
        if enable {
            *wcr |= 0b1 << 3;
        } else {
            *wcr &= !(0b1 << 3);
        }
    }
}

pub fn ltdc(enable: bool) {
    let wcr = DSI_WCR as *mut u32;

    unsafe {
        if enable {
            *wcr |= 0b1 << 2;
        } else {
            *wcr &= !(0b1 << 2);
        }
    }
}