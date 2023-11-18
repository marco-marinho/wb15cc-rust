#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

global_asm!(include_str!("startup/startup_stm32wb15ccux.s"));

const RCC_ADDR: usize = 0x5800_004C;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let rcc = unsafe { &mut *(RCC_ADDR as *mut u32) };
    *rcc |= 1u32<<1;
    let gpiob_moder = unsafe { &mut *(0x4800_0400 as *mut u32) };
    *gpiob_moder &= !(3u32<<10);
    *gpiob_moder |= 1u32<<10;
    let gpiob_odr = unsafe { &mut *(0x4800_0414 as *mut u32) };
    loop {
        *gpiob_odr ^= 1<<5;
        for _i in 0..10000{}
    }
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop{}
}