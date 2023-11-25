use crate::drivers::gpio::GPIOPort;
use crate::drivers::register::Register;

#[repr(C)]
pub struct RCC {
    pub cr: Register<u32>,
    // RCC clock  Control Register,                                                    Address offset: 0x00
    pub icscr: Register<u32>,
    // RCC Internal Clock Sources Calibration Register,                                Address offset: 0x04
    pub cfgr: Register<u32>,
    // RCC Clocks Configuration Register,                                              Address offset: 0x08
    pub pllcfgr: Register<u32>,
    // RCC System PLL configuration Register,                                          Address offset: 0x0C
    pub reserved0: [Register<u32>; 2],
    // Reserved,                                                                     Address offset: 0x10-0x14
    pub cier: Register<u32>,
    // RCC Clock Interrupt Enable Register,                                            Address offset: 0x18
    pub cifr: Register<u32>,
    // RCC Clock Interrupt Flag Register,                                              Address offset: 0x1C
    pub cicr: Register<u32>,
    // RCC Clock Interrupt Clear Register,                                             Address offset: 0x20
    pub smpscr: Register<u32>,
    // RCC SMPS step-down converter control register,                                  Address offset: 0x24
    pub ahb1rstr: Register<u32>,
    // RCC AHB1 peripheral reset register,                                             Address offset: 0x28
    pub ahb2rstr: Register<u32>,
    // RCC AHB2 peripheral reset register,                                             Address offset: 0x2C
    pub ahb3rstr: Register<u32>,
    // RCC AHB3 & AHB4 peripheral reset register,                                      Address offset: 0x30
    pub reserved1: Register<u32>,
    // Reserved,                                                                       Address offset: 0x34
    pub apb1rstr1: Register<u32>,
    // RCC APB1 peripheral reset register 1,                                           Address offset: 0x38
    pub apb1rstr2: Register<u32>,
    // RCC APB1 peripheral reset register 2,                                           Address offset: 0x3C
    pub apb2rstr: Register<u32>,
    // RCC APB2 peripheral reset register,                                             Address offset: 0x40
    pub apb3rstr: Register<u32>,
    // RCC APB3 peripheral reset register,                                             Address offset: 0x44
    pub ahb1enr: Register<u32>,
    // RCC AHB1 peripheral clocks enable register,                                     Address offset: 0x48
    pub ahb2enr: Register<u32>,
    // RCC AHB2 peripheral clocks enable register,                                     Address offset: 0x4C
    pub ahb3enr: Register<u32>,
    // RCC AHB3 & AHB4 peripheral clocks enable register,                              Address offset: 0x50
    pub reserved2: Register<u32>,
    // Reserved,                                                                       Address offset: 0x54
    pub apb1enr1: Register<u32>,
    // RCC APB1 peripheral clocks enable register 1,                                   Address offset: 0x58
    pub apb1enr2: Register<u32>,
    // RCC APB1 peripheral clocks enable register 2,                                   Address offset: 0x5C
    pub apb2enr: Register<u32>,
    // RCC APB2 peripheral clocks enable register,                                     Address offset: 0x60
    pub reserved3: Register<u32>,
    // Reserved,                                                                       Address offset: 0x64
    pub ahb1smenr: Register<u32>,
    // RCC AHB1 peripheral clocks enable in sleep and stop modes register,             Address offset: 0x68
    pub ahb2smenr: Register<u32>,
    // RCC AHB2 peripheral clocks enable in sleep and stop modes register,             Address offset: 0x6C
    pub ahb3smenr: Register<u32>,
    // RCC AHB3 & AHB4 peripheral clocks enable in sleep and stop modes register,      Address offset: 0x70
    pub reserved4: Register<u32>,
    // Reserved,                                                                       Address offset: 0x74
    pub apb1smenr1: Register<u32>,
    // RCC APB1 peripheral clocks enable in sleep mode and stop modes register 1,      Address offset: 0x78
    pub apb1smenr2: Register<u32>,
    // RCC APB1 peripheral clocks enable in sleep mode and stop modes register 2,      Address offset: 0x7C
    pub apb2smenr: Register<u32>,
    // RCC APB2 peripheral clocks enable in sleep mode and stop modes register,        Address offset: 0x80
    pub reserved5: Register<u32>,
    // Reserved,                                                                       Address offset: 0x84
    pub ccipr: Register<u32>,
    // RCC Peripherals Clock Configuration Independent Register,                       Address offset: 0x88
    pub reserved6: Register<u32>,
    // Reserved,                                                                       Address offset: 0x8C
    pub bdcr: Register<u32>,
    // RCC Backup Domain Control Register,                                             Address offset: 0x90
    pub csr: Register<u32>,
    // RCC Control and Status Register,                                                Address offset: 0x94
    pub crrcr: Register<u32>,
    // RCC Clock Recovery RC Register,                                                 Address offset: 0x98
    pub hsecr: Register<u32>,
    // RCC HSE Clock Register,                                                         Address offset: 0x9C
    pub reserved7: [Register<u32>; 26],
    // Reserved,                                                                     Address offset: 0xA0-0x104
    pub extcfgr: Register<u32>,
    // RCC Extended Clock Recovery Register,                                           Address offset: 0x108
    pub reserved8: [Register<u32>; 15],
    // Reserved,                                                                     Address offset: 0x10C-0x144
    pub c2ahb1enr: Register<u32>,
    // RRCC AHB1 peripheral CPU2 clocks enable register,                               Address offset: 0x148
    pub c2ahb2enr: Register<u32>,
    // RCC AHB2 peripheral CPU2 clocks enable register,                                Address offset: 0x14C
    pub c2ahb3enr: Register<u32>,
    // RCC AHB3 & AHB4 peripheral CPU2 clocks enable register,,                        Address offset: 0x150
    pub reserved9: Register<u32>,
    // Reserved,                                                                       Address offset: 0x154
    pub c2apb1enr1: Register<u32>,
    // RCC APB1 peripheral CPU2 clocks enable register 1,                              Address offset: 0x158
    pub c2apb1enr2: Register<u32>,
    // RCC APB1 peripheral CPU2 clocks enable register 2,                              Address offset: 0x15C
    pub c2apb2enr: Register<u32>,
    // RCC APB2 peripheral CPU2 clocks enable register 1,                              Address offset: 0x160
    pub c2apb3enr: Register<u32>,
    // RCC APB3 peripheral CPU2 clocks enable register 1,                              Address offset: 0x164
    pub c2ahb1smenr: Register<u32>,
    // RCC AHB1 peripheral CPU2 clocks enable in sleep and stop modes register,        Address offset: 0x168
    pub c2ahb2smenr: Register<u32>,
    // RCC AHB2 peripheral CPU2 clocks enable in sleep and stop modes register,        Address offset: 0x16C
    pub c2ahb3smenr: Register<u32>,
    // RCC AHB3 & AHB4 peripheral CPU2 clocks enable in sleep and stop modes register, Address offset: 0x170
    pub reserved10: Register<u32>,
    // Reserved,
    pub c2apb1smenr1: Register<u32>,
    // RCC APB1 peripheral CPU2 clocks enable in sleep mode and stop modes register 1, Address offset: 0x178
    pub c2apb1smenr2: Register<u32>,
    // RCC APB1 peripheral CPU2 clocks enable in sleep mode and stop modes register 2, Address offset: 0x17C
    pub c2apb2smenr: Register<u32>,
    // RCC APB2 peripheral CPU2 clocks enable in sleep mode and stop modes register,   Address offset: 0x180
    pub c2apb3smenr: Register<u32>,  // RCC APB3 peripheral CPU2 clocks enable in sleep mode and stop modes register,   Address offset: 0x184
}

impl RCC {
    pub fn enable_gpio(&self, port: GPIOPort) {
        let value = self.ahb2enr.get();
        match port {
            GPIOPort::A => { self.ahb2enr.set(value | 1) }
            GPIOPort::B => { self.ahb2enr.set(value | 1 << 1) }
            GPIOPort::C => { self.ahb2enr.set(value | 1 << 2) }
            GPIOPort::E => { self.ahb2enr.set(value | 1 << 4) }
            GPIOPort::H => { self.ahb2enr.set(value | 1 << 7) }
        }
    }

    pub fn enable_usart_1(&mut self) {
        let value = self.apb2enr.get();
        self.apb2enr.set( value | 1 << 14);
    }
    pub fn enable_adc(&mut self) {
        let value = self.ccipr.get();
        self.ccipr.set(value | (3 << 28));
        let value = self.apb2enr.get();
        self.apb2enr.set( value | 1 << 9);
        // self.apb2rstr.set(1 << 9);
    }

    pub fn set_msi(&mut self){
        while self.cr.get() & (1 << 1) == 0 {};
        let mut val = self.cr.get();
        val &= !((1+2+4+8) << 4);
        val |= (8+2) << 4;
        self.cr.set(val);
        self.smpscr.set(257);
    }
}