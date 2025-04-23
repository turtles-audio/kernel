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

/// Defines the size of a porch in pixels and lines
pub struct Porch {
    pixels: u32,
    lines: u32,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        return Color {
            r, g, b, a: 0
        }
    }

    pub fn as_argb8(self: Self) -> u32 {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        let a = self.a as u32;

        return a << 24 | r << 16 | g << 8 | b;
    }

    pub fn as_rgb(self: Self) -> u32 {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;

        return r << 16 | g << 8 | b;
    }

    pub fn as_rgb565(self: Self) -> u32 {
        let r = self.r as u32 & 0b11111;
        let g = self.g as u32 & 0b111111;
        let b = self.b as u32 & 0b11111;

        return r << 11 | g << 6 | b;
    }


}

pub enum Polarity {
    High,
    Low
}

pub enum Format {

}

pub enum Side {
    Top, Bottom
}

pub struct Layer {
    side: Side,
    format: Format,
    width: u16,
    height: u16,
}

impl Layer {
    pub fn offset(self: Self) -> u32 {
        return match self.side {
            Side::Top => 0,
            Side::Bottom => 0x80,
        }
    }
}

impl From<Polarity> for u32 {
    fn from(polarity: Polarity) -> u32 {
        return match polarity {
            Polarity::High => 0b1,
            Polarity::Low => 0b0,
        };
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

pub fn layer(layer: Layer) {
    let gcr = LTDC_GCR as *mut u32;
    let offset = layer.offset();

    let lcr = (LTDC_LCR + offset) as *mut u32;
    let lwhpcr = (LTDC_LWHPCR + offset) as *mut u32;
    let lwvpcr = (LTDC_LWVPCR + offset) as *mut u32;
    let lpfcr = (LTDC_LPFCR + offset) as *mut u32;


    unsafe {
        // Enable the layer
        *lcr |= 0b1;

        // Finally, enable the LTDC with the enable bit
        *gcr |= 0b1;
    }
}
