const LTDC_BASE: u32 = 0x50001000;

const LTDC_SSCR: u32 = LTDC_BASE + 0x8;
const LTDC_BPCR: u32 = LTDC_BASE + 0xC;
const LTDC_AWCR: u32 = LTDC_BASE + 0x10;
const LTDC_TWCR: u32 = LTDC_BASE + 0x14;
const LTDC_GCR: u32 = LTDC_BASE + 0x18;

fn back_porch() {
    let a = LTDC_GCR;
    let b = LTDC_GCR;
}
