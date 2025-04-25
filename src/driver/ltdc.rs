use crate::video::frame::{Polarity, Color, Layer};

use super::gpio;

const LTDC_BASE: u32 = 0x50001000;

const LTDC_SSCR: u32 = LTDC_BASE + 0x8;
const LTDC_BPCR: u32 = LTDC_BASE + 0xC;
const LTDC_AWCR: u32 = LTDC_BASE + 0x10;
const LTDC_TWCR: u32 = LTDC_BASE + 0x14;
const LTDC_GCR: u32 = LTDC_BASE + 0x18;
const LTDC_BCCR: u32 = LTDC_BASE + 0x2C;
const LTDC_IER: u32 = LTDC_BASE + 0x34;
const LTDC_LIPCR: u32 = LTDC_BASE + 0x40;

const LTDC_LCR: u32 = LTDC_BASE + 0x84;
const LTDC_LWHPCR: u32 = LTDC_BASE + 0x88;
const LTDC_LWVPCR: u32 = LTDC_BASE + 0x8C;
const LTDC_LPFCR:u32  = LTDC_BASE + 0x94;

const LTDC_LCFBAR: u32 = LTDC_BASE + 0xAC;
const LTDC_LCFBLR: u32 = LTDC_BASE + 0xB0;
const LTDC_LCFBLNR: u32 = LTDC_BASE + 0xB4;

/// Defines the size of a porch in pixels and lines
pub struct Porch {
    pixels: u32,
    lines: u32,
}

impl Porch {
    pub fn new(width: u32, height: u32) -> Porch {
        return Porch {
            pixels: width,
            lines: height,
        };
    }
}

/// LTDC Init Sequence
pub fn init() {
    let gcr = LTDC_GCR as *mut u32;

    gpio::toggle(gpio::Pin::J12, gpio::State::High).unwrap();
    gpio::speed(gpio::Pin::J12, gpio::Speed::High);
    gpio::write(gpio::Pin::J12, gpio::State::High).unwrap();

    unsafe {
        *gcr |= 0b1;
    }
}

/// Configure LTDC timings
pub fn timing(sync: Porch, back: Porch, active: Porch, front: Porch) {
    let sscr = LTDC_SSCR as *mut u32;
    let bpcr = LTDC_BPCR as *mut u32;
    let awcr = LTDC_AWCR as *mut u32;
    let twcr = LTDC_TWCR as *mut u32;    

    unsafe {
        // LTDC synchronization size configuration
        *sscr = (sync.pixels - 1) << 16; // Top 16 bits
        *sscr |= sync.lines - 1; // Bottom 16 bits
        
        // LTDC back porch configuration
        *bpcr = (sync.pixels + back.pixels - 1) << 16;
        *bpcr |= sync.lines + back.lines - 1;
    
        // LTDC active width configuration
        *awcr = (sync.pixels + back.pixels + active.pixels - 1) << 16;
        *awcr |= sync.lines + back.lines + active.lines - 1;

        // LTDC total width configuration
        *twcr = (sync.pixels + back.pixels + active.pixels + front.pixels - 1) << 16;
        *twcr |= sync.lines + back.lines + active.lines + front.lines - 1;
    }
}

/// Configure LTDC polarity
pub fn polarity(vertical: Polarity, horizontal: Polarity, clock: Polarity, data: Polarity) {
    let gcr = LTDC_GCR as *mut u32;

    unsafe {
        *gcr |= u32::from(vertical) << 30; // Vertical polarity
        *gcr |= u32::from(horizontal) << 31; // Horizontal polarity
        *gcr |= u32::from(data) << 29; // Data Enable polarity
        *gcr |= u32::from(clock) << 28; // Pixel Clock polarity
    }
}

/// Configure LTDC interrupts
pub fn interrupts(height: u32) {
    let ier = LTDC_IER as *mut u32;
    let lipcr = LTDC_LIPCR as *mut u32;
    
    unsafe {
        *ier |= 0b1111; // Enable all interrupt registers
        *lipcr = height - 1; // Set line interrupt position control register
    }
}

pub fn background(color: Color) {
    let bccr = LTDC_BCCR as *mut u32;

    unsafe {
        *bccr = color.as_rgb();
    }
}

pub fn layer(layer: &Layer) {
    let offset = layer.offset();

    let x = layer.x as u32 & 0x3FF;
    let y = layer.y as u32 & 0x3FF;

    let width = layer.width as u32 & 0x3FF;
    let height = layer.height as u32 & 0x3FF;

    let lcr = (LTDC_LCR + offset) as *mut u32;
    let lwhpcr = (LTDC_LWHPCR + offset) as *mut u32;
    let lwvpcr = (LTDC_LWVPCR + offset) as *mut u32;
    let lpfcr = (LTDC_LPFCR + offset) as *mut u32;
    let lcfbar = (LTDC_LCFBAR + offset) as *mut u32;
    let lcfblr = (LTDC_LCFBLR + offset) as *mut u32;
    let lcfblnr = (LTDC_LCFBLNR + offset) as *mut u32;

    if !layer.enabled {
        unsafe {
            *lcr = 0b0;
        }

        return;
    } else {
        unsafe {
            *lwhpcr = (x + width) << 16 | x;
            *lwvpcr = (y + height) << 16 | y;
            *lpfcr = 0b1;

            *lcfbar = 0x30094000;
            *lcfblr = width * 3;
            *lcfblnr = height;

            // Enable the layer
            *lcr |= 0b1;
        }
    }
}