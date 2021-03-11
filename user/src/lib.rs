#![no_std]
#![feature(linkage)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod syscall;
mod lang_items;
pub mod console;


/*******************************************************
 *                user mode entry                      *
 *******************************************************/
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable code!");
}


#[linkage = "weak"]
#[no_mangle]
fn main() -> i32{
    panic!("can't find main function");
}


/*******************************************************
 *                      api                            *
 *******************************************************/
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}


use crate::syscall::*;


pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}


pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}