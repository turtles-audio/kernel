use crate::driver::ltdc;

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
    RGB
}

pub enum Side {
    Top, Bottom
}

pub struct Layer {
    pub side: Side,
    pub format: Format,

    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    
    pub enabled: bool,
}

impl Layer {
    pub fn new(width: u16, height: u16, side: Side) -> Layer {
        return Layer {
            side,
            format: Format::RGB,
            
            width,
            height,
            x: 0,
            y: 0,

            enabled: false,
        }
    }

    pub fn offset(self: &Self) -> u32 {
        return match self.side {
            Side::Top => 0,
            Side::Bottom => 0x80,
        }
    }

    pub fn enable(self: &mut Self) {
        self.enabled = true;
        ltdc::layer(self);
    }

    pub fn disable(self: &mut Self) {
        self.enabled = false;
        ltdc::layer(self);
    }

    pub fn set_side(self: &mut Self, side: Side) {
        self.side = side;
    }

    pub fn set_format(self: &mut Self, format: Format) {
        self.format = format;
    }

    pub fn set_size(self: &mut Self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn set_position(self: &mut Self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
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