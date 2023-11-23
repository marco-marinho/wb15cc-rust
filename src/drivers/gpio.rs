use crate::drivers::register::Register;

#[repr(C)]
pub struct GPIO {
    pub moder: Register<u32>,
    // GPIO port mode register,               Address offset: 0x00
    pub otyper: Register<u32>,
    // GPIO port output type register,        Address offset: 0x04
    pub ospeedr: Register<u32>,
    // GPIO port output speed register,       Address offset: 0x08
    pub pupdr: Register<u32>,
    // GPIO port pull-up/pull-down register,  Address offset: 0x0C
    pub idr: Register<u32>,
    // GPIO port input data register,         Address offset: 0x10
    pub odr: Register<u32>,
    // GPIO port output data register,        Address offset: 0x14
    pub bsrr: Register<u32>,
    // GPIO port bit set/reset  register,     Address offset: 0x18
    pub lckr: Register<u32>,
    // GPIO port configuration lock register, Address offset: 0x1C
    pub afr: [Register<u32>; 2],
    // GPIO alternate function registers,     Address offset: 0x20-0x24
    pub brr: Register<u32>,
    // GPIO Bit Reset register,               Address offset: 0x28
}

impl GPIO {
    pub fn set_pin_mode(&mut self, mode: GPIOMode, pin: u32) {
        let mut current = self.moder.get();
        current &= !(3u32 << (pin * 2)); // Clean the bit pair for the respective pin
        let value = match mode {
            GPIOMode::Input => { 0u32 }
            GPIOMode::Output => { 1u32 }
            GPIOMode::Alternate => { 2u32 }
            GPIOMode::Analog => { 3u32 }
        };
        current |= value << (pin * 2); // Set mode
        self.moder.set(current);
    }

    pub fn toggle_pin(&mut self, pin: u32) {
        let val = self.odr.get()  ^ (1 << pin);
        self.odr.set(val);
    }

    pub fn reset_pin(&mut self, pin: u32) {
        self.bsrr.set(1 << (pin + 16));
    }

    pub fn set_pin(&mut self, pin: u32){
        self.bsrr.set(1 << pin );
    }

    pub fn read_pin(&self, pin: u32) -> u32{
        self.idr.get() & (1 << pin)
    }

    pub fn set_alternate_function(&mut self, pin: u32, function: u32) {
        if function > 15 {return}
        let reg: & Register<u32>;
        let offset: u32;
        if pin < 8 {
            reg = &self.afr[0];
            offset = pin * 4;
        } else {
            reg = &self.afr[1];
            offset = (pin - 8) * 4;
        };
        let mut value = reg.get();
        value &= !(15u32 << offset);
        value |= function << offset;
        reg.set(value);
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
