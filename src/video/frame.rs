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
