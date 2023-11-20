#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::ptr::write_volatile;
use core::mem::zeroed;
use core::ptr::read;
use core::arch::asm;

mod drivers;

global_asm!(include_str!("startup/startup.s"));
#[no_mangle]
pub unsafe extern "C" fn Reset_Handler() {
    extern "C" {
        // These symbols come from the linker file `*****.ld`
        static mut _sbss: u32; // Start of .bss section
        static mut _ebss: u32; // End of .bss section
        static mut _sdata: u32; // Start of .data section
        static mut _edata: u32; // End of .data section
        static _sidata: u32; // Start of .rodata section
        static mut __preinit_array_start: extern fn();
        static mut __preinit_array_end: extern fn();
        static mut __init_array_start: extern fn();
        static mut __init_array_end: extern fn();
    }

    // Initialize (Zero) BSS
    unsafe {
        let mut sbss: *mut u32 = &mut _sbss;
        let ebss: *mut u32 = &mut _ebss;

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize Data
    unsafe {
        let mut sdata: *mut u32 = &mut _sdata;
        let edata: *mut u32 = &mut _edata;
        let mut sidata: *const u32 = &_sidata;

        while sdata < edata {
            write_volatile(sdata, read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }

    // Initialize global objects (C++)
    unsafe {
        let mut preinit_start: *mut extern fn() = &mut __preinit_array_start;
        let preinit_end: *mut extern fn() = &mut __preinit_array_end;
        while preinit_start < preinit_end {
            (*preinit_start)();
            preinit_start = preinit_start.offset(1);
        }

        let mut init_start: *mut extern fn() = &mut __init_array_start;
        let init_end: *mut extern fn() = &mut __init_array_end;
        while init_start < init_end {
            (*init_start)();
            init_start = init_start.offset(1);
        }
    }

    // Enable FPU
    unsafe{
        asm!("ldr r0, =0xE000ED88",
        "ldr r1, =(0b1111 << 20)",
        "ldr r2, [r0]",
        "orr r2, r2, r1",
        "str r2, [r0]",
        "dsb",
        "isb");
    }

    // Call main function
    _start()
}

fn _start() -> ! {
    let rcc = unsafe {&mut *(0x5800_0000 as *mut drivers::rcc::RCC)};
    rcc.enable_gpio(drivers::gpio::GPIOPort::B);
    rcc.enable_gpio(drivers::gpio::GPIOPort::A);
    let gpioa = unsafe { &mut *(0x4800_0000 as *mut drivers::gpio::GPIO) };
    let gpiob = unsafe { &mut *(0x4800_0400 as *mut drivers::gpio::GPIO) };
    gpiob.set_pin_mode(drivers::gpio::GPIOMode::Output, 5);
    gpiob.set_pin_mode(drivers::gpio::GPIOMode::Output, 1);
    gpiob.set_pin_mode(drivers::gpio::GPIOMode::Output, 0);

    gpioa.set_pin_mode(drivers::gpio::GPIOMode::Input, 6);

    loop {
        if !gpioa.read_pin(6) {
            gpiob.set_pin(5);
            gpiob.set_pin(1);
            gpiob.set_pin(0);
        }
        else {
            gpiob.reset_pin(5);
            gpiob.reset_pin(1);
            gpiob.reset_pin(0);
        }
    }
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop{}
}
