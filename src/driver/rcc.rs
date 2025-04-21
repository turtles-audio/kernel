/// Address to the base of the RCC registers
const RCC_BASE: u32 = 0x58024400;

// RCC APB Registers
const RCC_APB1HENR: u32 = RCC_BASE + 0xEC;
const RCC_APB1LENR: u32 = RCC_BASE + 0xE8;
const RCC_APB2ENR: u32 = RCC_BASE + 0xF0;
const RCC_APB3ENR: u32 = RCC_BASE + 0xE4;
const RCC_APB4ENR: u32 = RCC_BASE + 0xF4;

// RCC AHB Registers
const RCC_AHB1ENR: u32 = RCC_BASE + 0xD8;
const RCC_AHB2ENR: u32 = RCC_BASE + 0xDC;
const RCC_AHB3ENR: u32 = RCC_BASE + 0xD4;
const RCC_AHB4ENR: u32 = RCC_BASE + 0xE0;

// Enable LTDC timer
pub fn ltdc() -> Result<(), ()> {
    // Pointer to APB3EN register
    let reg: *mut u32 = RCC_APB3ENR as *mut u32;

    unsafe {
        // Toggle the LTDC flag in APB3ENR
        *reg |= 1 << 3;
        
        // Toggle the DSI flag in APB3ENR
        *reg |= 1 << 4;

        // Check if the change is saved
        if 1 & (*reg >> 3) == 1 {
           return Ok(());
        }
    }

    // Changed bits do not match
    return Err(());
}

// Enable GPIO timers
pub fn gpio() -> Result<(), ()> {
    // Pointer to AHB4EN register
    let reg: *mut u32 = RCC_AHB4ENR as *mut u32;
    
    // GPIO flag bits for A-K
    const FLAG: u32 = 0b11111111111;

    unsafe {
        // Toggle GPIO timers for GPIO A-K
        *reg |= FLAG;
        
        // Check if the change is saved
        if FLAG & *reg == FLAG {
            return Ok(())
        }
    }

    // Changed bits do not match
    return Err(());
}

/// Initialize all available peripheral timers
pub fn peripherals() {
    // Expect timers to start
    ltdc().expect("Failed to initialize peripheral LTDC");
    gpio().expect("Failed to initialize peripheral GPIO");
}
