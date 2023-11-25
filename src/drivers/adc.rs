use core::ptr::read_volatile;
use crate::drivers::register::Register;
use crate::drivers::rcc::RCC;

#[repr(C)]
pub struct ADC {
    pub isr: Register<u32>,
    // ADC interrupt and status register,             Address offset: 0x00
    pub ier: Register<u32>,
    // ADC interrupt enable register,                 Address offset: 0x04
    pub cr: Register<u32>,
    // ADC control register,                          Address offset: 0x08
    pub cfgr1: Register<u32>,
    // ADC configuration register 1,                  Address offset: 0x0C
    pub cfgr2: Register<u32>,
    // ADC configuration register 2,                  Address offset: 0x10
    pub smpr: Register<u32>,
    // ADC sampling time register,                    Address offset: 0x14
    pub reserved1: Register<u32>,
    // Reserved,                                                      0x18
    pub reserved2: Register<u32>,
    // Reserved,                                                      0x1C
    pub tr: Register<u32>,
    // ADC analog watchdog 1 threshold register,      Address offset: 0x20
    pub reserved3: Register<u32>,
    // Reserved,                                                      0x24
    pub chselr: Register<u32>,
    // ADC group regular sequencer register,          Address offset: 0x28
    pub reserved4: Register<u32>,
    // Reserved,                                                      0x2C
    pub reserved5: [Register<u32>; 4],
    // Reserved,                                               0x30 - 0x3C
    pub dr: Register<u32>,
    // ADC group regular data register,               Address offset: 0x40
    pub reserved6: [Register<u32>; 23],
    //Reserved,                                               0x44 - 0x9C
    pub reserved7: Register<u32>,
    // Reserved,                                                      0xA0
    pub reserved8: Register<u32>,
    // Reserved,                                                      0xA4
    pub reserved9: [Register<u32>; 3],
    // Reserved,                                               0xA8 - 0xB0
    pub calfact: Register<u32>,
    // ADC Calibration factor register,               Address offset: 0xB4
}

pub struct ADCCCR{
    reserved1 : Register<u32>,    // Reserved,                                      Address offset: ADC1 base address + 0x300 */
    reserved2: Register<u32>,   // Reserved,                                      Address offset: ADC1 base address + 0x304 */
    pub ccr: Register<u32>,         // ADC common configuration register,             Address offset: ADC1 base address + 0x308 */
    reserved3: Register<u32>,    // Reserved,                                      Address offset: ADC1 base address + 0x30C */
}

impl ADC{

    pub fn get_ccr(&mut self) -> &mut ADCCCR{
        unsafe {&mut *((self as *const ADC as usize + 0x300) as *mut ADCCCR)}
    }

    pub fn wait_for_ccrdy(&mut self) {
        while self.isr.get() & (1 << 13) == 0 {}
        // let val = self.isr.get();
        // self.isr.set(val & !(1<<13));
    }

    pub fn wait_for_adrdy(&mut self) {
        while self.isr.get() & (1 << 0) == 0 {}
        let val = self.isr.get();
        self.isr.set(val & !(1<<0));
    }
    pub fn setup(&mut self, rcc: &mut RCC){
        rcc.enable_adc();
        self.cr.set(1 << 28);
        let ccr = self.get_ccr();
        let ccr_val = ccr.ccr.get();
        ccr.ccr.set(ccr_val | (1 << 23));
        let chsel_val = self.chselr.get();
        self.chselr.set(chsel_val | (1 << 12));
        self.wait_for_ccrdy();
        let val = self.cr.get();
        self.cr.set(val | 1);
        self.wait_for_adrdy();
    }

    pub fn start_conversion(&mut self){
        let cr_val = self.cr.get();
        self.cr.set(cr_val | (1<<2));
    }

    pub fn read_conversion(&mut self) -> u32{
        while self.isr.get() & (1 << 2) == 0 {}
        self.dr.get()
    }

    pub fn convert_temp(data: u32) -> f32{
        let p1 = 0x1FFF_75A8 as *const u32;
        let tcal_1 = unsafe {read_volatile(p1)};
        let p2 = 0x1FFF_75CA as *const u32;
        let tcal_2 = unsafe {read_volatile(p2)};
        let a = (130f32 - 30f32)/(tcal_2 - tcal_1) as f32;
        let b = (data as f32 - tcal_1 as f32);
        a * b + 30f32
    }

}