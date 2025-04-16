use super::led::Color;

const GPIO_BASE: u32 = 0x58020000;

const GPIOA_BASE: u32 = GPIO_BASE;
const GPIOB_BASE: u32 = GPIO_BASE + 0x400;
const GPIOC_BASE: u32 = GPIO_BASE + 0x800;
const GPIOD_BASE: u32 = GPIO_BASE + 0xC00;
const GPIOE_BASE: u32 = GPIO_BASE + 0x1000;
const GPIOF_BASE: u32 = GPIO_BASE + 0x1400;
const GPIOG_BASE: u32 = GPIO_BASE + 0x1800;
const GPIOH_BASE: u32 = GPIO_BASE + 0x1C00;
const GPIOI_BASE: u32 = GPIO_BASE + 0x2000;
const GPIOJ_BASE: u32 = GPIO_BASE + 0x2400;
const GPIOK_BASE: u32 = GPIO_BASE + 0x2800;

const GPIO_MODER: u32 = 0x00;
const GPIO_OTYPER: u32 = 0x04;
const GPIO_OSPEEDR: u32 = 0x08;
const GPIO_PUPDR: u32 = 0x0C;
const GPIO_IDR: u32 = 0x10;
const GPIO_ODR: u32 = 0x14;
const GPIO_BSSR: u32 = 0x18;
const GPIO_LCKR: u32 = 0x1C;
const GPIO_AFRL: u32 = 0x20;
const GPIO_AFRH: u32 = 0x24;

pub enum State {
    High,
    Low,
}

impl State {
    pub fn bit(state: Self) -> u32 {
        return match state {
            State::High => 0b1,
            State::Low => 0b0,
        };
    }
}

pub enum Pin {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    B11,
    B12,
    B13,
    B14,
    B15,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    E0,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    E9,
    E10,
    E11,
    E12,
    E13,
    E14,
    E15,
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    G0,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    G9,
    G10,
    G11,
    G12,
    G13,
    G14,
    G15,
    H0,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
    H10,
    H11,
    H12,
    H13,
    H14,
    H15,
    I0,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
    I10,
    I11,
    I12,
    I13,
    I14,
    I15,
    J0,
    J1,
    J2,
    J3,
    J4,
    J5,
    J6,
    J7,
    J8,
    J9,
    J10,
    J11,
    J12,
    J13,
    J14,
    J15,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    K10,
    K11,
    K12,
    K13,
    K14,
    K15,
}

impl From<Color> for Pin {
    fn from(item: Color) -> Self {
        return match item {
            Color::Green => Pin::I12,
            Color::Orange => Pin::I13,
            Color::Red => Pin::I14,
            Color::Blue => Pin::I15,
        };
    }
}

pub fn register(pin: &Pin) -> u32 {
    return match pin {
        Pin::A0
        | Pin::A1
        | Pin::A2
        | Pin::A3
        | Pin::A4
        | Pin::A5
        | Pin::A6
        | Pin::A7
        | Pin::A8
        | Pin::A9
        | Pin::A10
        | Pin::A11
        | Pin::A12
        | Pin::A13
        | Pin::A14
        | Pin::A15 => GPIOA_BASE,
        Pin::B0
        | Pin::B1
        | Pin::B2
        | Pin::B3
        | Pin::B4
        | Pin::B5
        | Pin::B6
        | Pin::B7
        | Pin::B8
        | Pin::B9
        | Pin::B10
        | Pin::B11
        | Pin::B12
        | Pin::B13
        | Pin::B14
        | Pin::B15 => GPIOB_BASE,
        Pin::C0
        | Pin::C1
        | Pin::C2
        | Pin::C3
        | Pin::C4
        | Pin::C5
        | Pin::C6
        | Pin::C7
        | Pin::C8
        | Pin::C9
        | Pin::C10
        | Pin::C11
        | Pin::C12
        | Pin::C13
        | Pin::C14
        | Pin::C15 => GPIOC_BASE,
        Pin::D0
        | Pin::D1
        | Pin::D2
        | Pin::D3
        | Pin::D4
        | Pin::D5
        | Pin::D6
        | Pin::D7
        | Pin::D8
        | Pin::D9
        | Pin::D10
        | Pin::D11
        | Pin::D12
        | Pin::D13
        | Pin::D14
        | Pin::D15 => GPIOD_BASE,
        Pin::E0
        | Pin::E1
        | Pin::E2
        | Pin::E3
        | Pin::E4
        | Pin::E5
        | Pin::E6
        | Pin::E7
        | Pin::E8
        | Pin::E9
        | Pin::E10
        | Pin::E11
        | Pin::E12
        | Pin::E13
        | Pin::E14
        | Pin::E15 => GPIOE_BASE,
        Pin::F0
        | Pin::F1
        | Pin::F2
        | Pin::F3
        | Pin::F4
        | Pin::F5
        | Pin::F6
        | Pin::F7
        | Pin::F8
        | Pin::F9
        | Pin::F10
        | Pin::F11
        | Pin::F12
        | Pin::F13
        | Pin::F14
        | Pin::F15 => GPIOF_BASE,
        Pin::G0
        | Pin::G1
        | Pin::G2
        | Pin::G3
        | Pin::G4
        | Pin::G5
        | Pin::G6
        | Pin::G7
        | Pin::G8
        | Pin::G9
        | Pin::G10
        | Pin::G11
        | Pin::G12
        | Pin::G13
        | Pin::G14
        | Pin::G15 => GPIOG_BASE,
        Pin::H0
        | Pin::H1
        | Pin::H2
        | Pin::H3
        | Pin::H4
        | Pin::H5
        | Pin::H6
        | Pin::H7
        | Pin::H8
        | Pin::H9
        | Pin::H10
        | Pin::H11
        | Pin::H12
        | Pin::H13
        | Pin::H14
        | Pin::H15 => GPIOH_BASE,
        Pin::I0
        | Pin::I1
        | Pin::I2
        | Pin::I3
        | Pin::I4
        | Pin::I5
        | Pin::I6
        | Pin::I7
        | Pin::I8
        | Pin::I9
        | Pin::I10
        | Pin::I11
        | Pin::I12
        | Pin::I13
        | Pin::I14
        | Pin::I15 => GPIOI_BASE,
        Pin::J0
        | Pin::J1
        | Pin::J2
        | Pin::J3
        | Pin::J4
        | Pin::J5
        | Pin::J6
        | Pin::J7
        | Pin::J8
        | Pin::J9
        | Pin::J10
        | Pin::J11
        | Pin::J12
        | Pin::J13
        | Pin::J14
        | Pin::J15 => GPIOJ_BASE,
        Pin::K0
        | Pin::K1
        | Pin::K2
        | Pin::K3
        | Pin::K4
        | Pin::K5
        | Pin::K6
        | Pin::K7
        | Pin::K8
        | Pin::K9
        | Pin::K10
        | Pin::K11
        | Pin::K12
        | Pin::K13
        | Pin::K14
        | Pin::K15 => GPIOK_BASE,
    };
}

pub fn write(pin: Pin, state: State) -> Result<(), ()> {
    let odr = (register(&pin) + GPIO_ODR) as *mut u32;

    unsafe {
        match state {
            State::High => *odr |= 0b1 << (pin as u32) % 16,
            State::Low => *odr &= !(0b1 << (pin as u32) % 16),
        }
    }

    return Ok(());
}

pub fn read(pin: Pin) -> State {
    let idr: *mut u32 = (register(&pin) + GPIO_IDR) as *mut u32;

    unsafe {
        let bit: u32 = 0b1 & (*idr >> (pin as u32) % 16);

        return match bit {
            0b0 => State::Low,
            _ => State::High,
        };
    }
}

pub fn toggle(pin: Pin, state: State) -> Result<(), ()> {
    let moder: *mut u32 = (register(&pin) + GPIO_MODER) as *mut u32;
    let pin_index: u32 = pin as u32;

    unsafe {
        *moder &= !(0b11 << (pin_index * 2));
        *moder |= State::bit(state) << (pin_index * 2);
    }

    return Ok(());
}
