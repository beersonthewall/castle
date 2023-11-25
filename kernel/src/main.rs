#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! { intrinsics::abort() }
