const LTDC_BASE: u32 = 0x50001000;

const LTDC_SSCR: u32 = LTDC_BASE + 0x8;
const LTDC_BPCR: u32 = LTDC_BASE + 0xC;
const LTDC_AWCR: u32 = LTDC_BASE + 0x10;
const LTDC_TWCR: u32 = LTDC_BASE + 0x14;
const LTDC_GCR: u32 = LTDC_BASE + 0x18;

pub struct Porch {
    pixels: u32,
    lines: u32,
}

pub fn timing(sync: Porch, back: Porch, active: Porch, front: Porch) {
    let sscr = LTDC_SSCR as *mut u32;
    let bpcr = LTDC_BPCR as *mut u32;
    let awcr = LTDC_AWCR as *mut u32;

    unsafe {
        // LTDC synchronization size configuration
        *sscr = (sync.pixels - 1) << 16; // Top 16 bits
        *sscr |= (sync.lines - 1); // Bottom 16 bits
        
        // LTDC back porch configuration
        *bpcr = (sync.pixels + back.pixels - 1) << 16;
        *bpcr |= (sync.lines + back.lines - 1);
    
        // LTDC 
    }
}
