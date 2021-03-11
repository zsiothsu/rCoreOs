#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(llvm_asm)]
#![allow(dead_code)]

#[macro_use]
mod sbicall;
mod console;
mod lang_items;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));
/*******************************************************
 *                    program                          *
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

fn show_seg() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
    }
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}

/*******************************************************
 *                      main                           *
 *******************************************************/
#[no_mangle]
pub fn rust_main() -> ! {
    show_seg();
    clear_bss();
    printkln!("\x1b[31mHello World!\x1b[0m");
    sbicall::shutdown();
}