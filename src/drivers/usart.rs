use crate::drivers::rcc::RCC;
use crate::drivers::gpio;
use crate::drivers::gpio::{GPIO, GPIOMode};
use crate::drivers::register::Register;

pub struct USART {
    pub cr1: Register<u32>,
    // USART Control register 1,                 Address offset: 0x00
    pub cr2: Register<u32>,
    // USART Control register 2,                 Address offset: 0x04
    pub cr3: Register<u32>,
    // USART Control register 3,                 Address offset: 0x08
    pub brr: Register<u32>,
    // USART Baud rate register,                 Address offset: 0x0C
    pub gtpr: Register<u32>,
    // USART Guard time and prescaler register,  Address offset: 0x10
    pub rtor: Register<u32>,
    // USART Receiver Time Out register,         Address offset: 0x14
    pub rqr: Register<u32>,
    // USART Request register,                   Address offset: 0x18
    pub isr: Register<u32>,
    // USART Interrupt and status register,      Address offset: 0x1C
    pub icr: Register<u32>,
    // USART Interrupt flag Clear register,      Address offset: 0x20
    pub rdr: Register<u32>,
    // USART Receive Data register,              Address offset: 0x24
    pub tdr: Register<u32>,
    // USART Transmit Data register,             Address offset: 0x28
    pub presc: Register<u32>,
    // USART Prescaler register,                 Address offset: 0x2C
}

fn set_bit(reg: &Register<u32>, bit: u32) {
    let mut curr = reg.get();
    curr |= 1 << bit;
    reg.set(curr);
}

impl USART {
    fn set_baud_rate(&mut self, clk: u32, baud_rate: u32) {
        self.brr.set((clk + (baud_rate / 2)) / baud_rate);
    }
    fn enable_tx(&self) {
        set_bit(&self.cr1, 3);
    }
    fn enable_rx(&self) {
        set_bit(& self.cr1, 2);
    }
    fn enable(&self) {
        set_bit(&self.cr1, 0);
    }
    pub fn setup(&mut self, rcc: &mut RCC, gpioa: &mut GPIO, gpiob: &mut GPIO, clk: u32, baud: u32) {
        rcc.enable_gpio(gpio::GPIOPort::A);
        rcc.enable_gpio(gpio::GPIOPort::B);
        gpioa.set_pin_mode(GPIOMode::Alternate, 9);
        gpioa.set_pin_mode(GPIOMode::Alternate, 10);
        gpioa.set_alternate_function(9, 7);
        gpioa.set_alternate_function(10, 7);
        gpiob.set_pin_mode(GPIOMode::Alternate, 6);
        gpiob.set_pin_mode(GPIOMode::Alternate, 7);
        gpiob.set_alternate_function(6, 7);
        gpiob.set_alternate_function(7, 7);
        rcc.enable_usart_1();
        self.set_baud_rate(clk, baud);
        self.enable_tx();
        self.enable_rx();
        self.enable();
    }
    pub fn write(&mut self, s: &str) {
        for c in s.chars() {
            while self.isr.get() & (1 << 7) == 0 {}
            self.tdr.set(c as u32 & 0xFF);
        }
        while self.isr.get() & (1 << 6) == 0 {}
    }
}

