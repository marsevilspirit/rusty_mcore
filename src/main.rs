#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(rusty_mcore::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_mcore::println;

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为编译器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
    // 默认命名为 `_start`

    println!("hello World{}", "!");
    println!("pluto fw");

    #[cfg(test)]
    test_main();

    loop {}
}

// 这个函数将在 panic 时被调用
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_mcore::test_panic_handler(info)
}
