#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::ptr::write_volatile;
use core::mem::zeroed;
use core::ptr::read;
use core::arch::asm;
use crate::drivers::gpio;

mod drivers;

global_asm!(include_str!("startup/startup.s"));
#[no_mangle]
pub extern "C" fn Reset_Handler() {
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
    let gpioa = unsafe { &mut *(0x4800_0000 as *mut gpio::GPIO) };
    let gpiob = unsafe { &mut *(0x4800_0400 as *mut gpio::GPIO) };
    let usart = unsafe { &mut * (0x4001_3800 as *mut drivers::usart::USART)};
    usart.setup_gpio(rcc, gpioa, gpio::GPIOPort::A);
    usart.setup(rcc, 4_000_000, 115200);
    usart.write("Waiting:\n\r");
    loop {
        let key = usart.read();
        if key == 's' {
            usart.write("\n\rGot s\n\r");
        }
        else {
            usart.write("\n\rGot something else\n\r");
        }
    }
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop{}
}
