#[repr(C)]
pub struct GPIO {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub odr: u32,
    pub bsrr: u32,
    pub lckr: u32,
    pub afr: [u32; 2],
    pub brr: u32
}

impl GPIO {
    pub fn set_pin_mode(&mut self, mode: GPIOMode, pin: u32) {
        self.moder &= !(3u32<<(pin * 2));
        let value = match mode {
            GPIOMode::Input => {0u32}
            GPIOMode::Output => {1u32}
            GPIOMode::Alternate => {2u32}
            GPIOMode::Analog => {3u32}
        };
        self.moder |= value << (pin * 2);
    }

    pub fn toggle_pin(&mut self, pin: u32){
        self.odr ^= 1 << pin;
    }
}

#[allow(dead_code)]
pub enum GPIOMode{
    Input,
    Output,
    Alternate,
    Analog,
}
