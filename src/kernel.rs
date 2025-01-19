#![no_std]
#![no_main]

mod sbi;
mod util;

use core::{arch::asm, panic::PanicInfo};
use util::memset;

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
fn kernel_main() {
    unsafe {
        let __bss_mut = __bss as *mut u8;
        memset(__bss_mut, 0, (__bss_end - __bss).into());
    }

    let s = "\n\nHello World!\n";
    for c in s.chars() {
        let _ = sbi::putchar(c);
    }

    unreachable!()
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() -> ! {
    asm!(
        "mv sp, {stack_top}
         j {kernel_main}",
         stack_top = in(reg) &__stack_top,
         kernel_main = sym kernel_main,
    );

    loop {}
}
