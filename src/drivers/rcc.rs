use core::ptr::{read_volatile, write_volatile};
use crate::drivers::gpio::GPIOPort;

#[repr(C)]
pub struct RCC{
    pub cr: u32,           // RCC clock  Control Register,                                                    Address offset: 0x00
    pub icscr: u32,        // RCC Internal Clock Sources Calibration Register,                                Address offset: 0x04
    pub cfgr: u32,         // RCC Clocks Configuration Register,                                              Address offset: 0x08
    pub pllcfgr: u32,      // RCC System PLL configuration Register,                                          Address offset: 0x0C
    pub reserved0: [u32; 2], // Reserved,                                                                     Address offset: 0x10-0x14
    pub cier: u32,         // RCC Clock Interrupt Enable Register,                                            Address offset: 0x18
    pub cifr: u32,         // RCC Clock Interrupt Flag Register,                                              Address offset: 0x1C
    pub cicr: u32,         // RCC Clock Interrupt Clear Register,                                             Address offset: 0x20
    pub smpscr: u32,       // RCC SMPS step-down converter control register,                                  Address offset: 0x24
    pub ahb1rstr: u32,     // RCC AHB1 peripheral reset register,                                             Address offset: 0x28
    pub ahb2rstr: u32,     // RCC AHB2 peripheral reset register,                                             Address offset: 0x2C
    pub ahb3rstr: u32,     // RCC AHB3 & AHB4 peripheral reset register,                                      Address offset: 0x30
    pub reserved1: u32,    // Reserved,                                                                       Address offset: 0x34
    pub apb1rstr1: u32,    // RCC APB1 peripheral reset register 1,                                           Address offset: 0x38
    pub apb1rstr2: u32,    // RCC APB1 peripheral reset register 2,                                           Address offset: 0x3C
    pub apb2rstr: u32,     // RCC APB2 peripheral reset register,                                             Address offset: 0x40
    pub apb3rstr: u32,     // RCC APB3 peripheral reset register,                                             Address offset: 0x44
    pub ahb1enr: u32,      // RCC AHB1 peripheral clocks enable register,                                     Address offset: 0x48
    pub ahb2enr: u32,      // RCC AHB2 peripheral clocks enable register,                                     Address offset: 0x4C
    pub ahb3enr: u32,      // RCC AHB3 & AHB4 peripheral clocks enable register,                              Address offset: 0x50
    pub reserved2: u32,    // Reserved,                                                                       Address offset: 0x54
    pub apb1enr1: u32,     // RCC APB1 peripheral clocks enable register 1,                                   Address offset: 0x58
    pub apb1enr2: u32,     // RCC APB1 peripheral clocks enable register 2,                                   Address offset: 0x5C
    pub apb2enr: u32,      // RCC APB2 peripheral clocks enable register,                                     Address offset: 0x60
    pub reserved3: u32,    // Reserved,                                                                       Address offset: 0x64
    pub ahb1smenr: u32,    // RCC AHB1 peripheral clocks enable in sleep and stop modes register,             Address offset: 0x68
    pub ahb2smenr: u32,    // RCC AHB2 peripheral clocks enable in sleep and stop modes register,             Address offset: 0x6C
    pub ahb3smenr: u32,    // RCC AHB3 & AHB4 peripheral clocks enable in sleep and stop modes register,      Address offset: 0x70
    pub reserved4: u32,    // Reserved,                                                                       Address offset: 0x74
    pub apb1smenr1: u32,   // RCC APB1 peripheral clocks enable in sleep mode and stop modes register 1,      Address offset: 0x78
    pub apb1smenr2: u32,   // RCC APB1 peripheral clocks enable in sleep mode and stop modes register 2,      Address offset: 0x7C
    pub apb2smenr: u32,    // RCC APB2 peripheral clocks enable in sleep mode and stop modes register,        Address offset: 0x80
    pub reserved5: u32,    // Reserved,                                                                       Address offset: 0x84
    pub ccipr: u32,        // RCC Peripherals Clock Configuration Independent Register,                       Address offset: 0x88
    pub reserved6: u32,    // Reserved,                                                                       Address offset: 0x8C
    pub bdcr: u32,         // RCC Backup Domain Control Register,                                             Address offset: 0x90
    pub csr: u32,          // RCC Control and Status Register,                                                Address offset: 0x94
    pub crrcr: u32,        // RCC Clock Recovery RC Register,                                                 Address offset: 0x98
    pub hsecr: u32,        // RCC HSE Clock Register,                                                         Address offset: 0x9C
    pub reserved7: [u32; 26],// Reserved,                                                                     Address offset: 0xA0-0x104
    pub extcfgr: u32,      // RCC Extended Clock Recovery Register,                                           Address offset: 0x108
    pub reserved8: [u32; 15],// Reserved,                                                                     Address offset: 0x10C-0x144
    pub c2ahb1enr: u32,    // RRCC AHB1 peripheral CPU2 clocks enable register,                               Address offset: 0x148
    pub c2ahb2enr: u32,    // RCC AHB2 peripheral CPU2 clocks enable register,                                Address offset: 0x14C
    pub c2ahb3enr: u32,    // RCC AHB3 & AHB4 peripheral CPU2 clocks enable register,,                        Address offset: 0x150
    pub reserved9: u32,    // Reserved,                                                                       Address offset: 0x154
    pub c2apb1enr1: u32,   // RCC APB1 peripheral CPU2 clocks enable register 1,                              Address offset: 0x158
    pub c2apb1enr2: u32,   // RCC APB1 peripheral CPU2 clocks enable register 2,                              Address offset: 0x15C
    pub c2apb2enr: u32,    // RCC APB2 peripheral CPU2 clocks enable register 1,                              Address offset: 0x160
    pub c2apb3enr: u32,    // RCC APB3 peripheral CPU2 clocks enable register 1,                              Address offset: 0x164
    pub c2ahb1smenr: u32,  // RCC AHB1 peripheral CPU2 clocks enable in sleep and stop modes register,        Address offset: 0x168
    pub c2ahb2smenr: u32,  // RCC AHB2 peripheral CPU2 clocks enable in sleep and stop modes register,        Address offset: 0x16C
    pub c2ahb3smenr: u32,  // RCC AHB3 & AHB4 peripheral CPU2 clocks enable in sleep and stop modes register, Address offset: 0x170
    pub reserved10: u32,   // Reserved,
    pub c2apb1smenr1: u32, // RCC APB1 peripheral CPU2 clocks enable in sleep mode and stop modes register 1, Address offset: 0x178
    pub c2apb1smenr2: u32, // RCC APB1 peripheral CPU2 clocks enable in sleep mode and stop modes register 2, Address offset: 0x17C
    pub c2apb2smenr: u32,  // RCC APB2 peripheral CPU2 clocks enable in sleep mode and stop modes register,   Address offset: 0x180
    pub c2apb3smenr: u32,  // RCC APB3 peripheral CPU2 clocks enable in sleep mode and stop modes register,   Address offset: 0x184
}

impl RCC {
    pub fn enable_gpio(&mut self, port: GPIOPort){
        let value = unsafe {read_volatile(&self.ahb2enr)};
        match port {
            GPIOPort::A => {unsafe{write_volatile(&mut self.ahb2enr, value | 1)}}
            GPIOPort::B => {unsafe{write_volatile(&mut self.ahb2enr, value | 1 << 1)}}
            GPIOPort::C => {unsafe{write_volatile(&mut self.ahb2enr, value | 1 << 2)}}
            GPIOPort::E => {unsafe{write_volatile(&mut self.ahb2enr, value | 1 << 4)}}
            GPIOPort::H => {unsafe{write_volatile(&mut self.ahb2enr, value | 1 << 7)}}
        }
    }
}