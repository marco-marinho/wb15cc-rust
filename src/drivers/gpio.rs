use core::ptr::write_volatile;
use core::ptr::read_volatile;

#[repr(C)]
pub struct GPIO {
    pub moder: u32,
    // GPIO port mode register,               Address offset: 0x00
    pub otyper: u32,
    // GPIO port output type register,        Address offset: 0x04
    pub ospeedr: u32,
    // GPIO port output speed register,       Address offset: 0x08
    pub pupdr: u32,
    // GPIO port pull-up/pull-down register,  Address offset: 0x0C
    pub idr: u32,
    // GPIO port input data register,         Address offset: 0x10
    pub odr: u32,
    // GPIO port output data register,        Address offset: 0x14
    pub bsrr: u32,
    // GPIO port bit set/reset  register,     Address offset: 0x18
    pub lckr: u32,
    // GPIO port configuration lock register, Address offset: 0x1C
    pub afr: [u32; 2],
    // GPIO alternate function registers,     Address offset: 0x20-0x24
    pub brr: u32,
    // GPIO Bit Reset register,               Address offset: 0x28
}

impl GPIO {
    pub fn set_pin_mode(&mut self, mode: GPIOMode, pin: u32) {
        let mut current = unsafe { read_volatile(&self.moder) };
        current &= !(3u32 << (pin * 2)); // Clean the bit pair for the respective pin
        let value = match mode {
            GPIOMode::Input => { 0u32 }
            GPIOMode::Output => { 1u32 }
            GPIOMode::Alternate => { 2u32 }
            GPIOMode::Analog => { 3u32 }
        };
        current |= value << (pin * 2); // Set mode
        unsafe { write_volatile(&mut self.moder, current) }
    }

    pub fn toggle_pin(&mut self, pin: u32) {
        self.odr ^= 1 << pin;
    }

    pub fn reset_pin(&mut self, pin: u32) {
        unsafe { write_volatile(&mut self.bsrr, 1 << (pin + 16)) }
    }

    pub fn set_pin(&mut self, pin: u32){
        unsafe { write_volatile(&mut self.bsrr, 1 << pin) }
    }

    pub fn read_pin(&self, pin: u32) -> bool{
        let reg = unsafe {read_volatile(&self.idr)};
        (reg & (1 << pin)) != 0
    }

}

pub enum GPIOMode {
    Input,
    Output,
    Alternate,
    Analog,
}

pub enum GPIOPort {
    A,
    B,
    C,
    E,
    H
}
