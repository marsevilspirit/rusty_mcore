#![no_std]
#![no_main]

use core::panic::PanicInfo;

// no_mangle经验篇能够禁用名称重整
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
