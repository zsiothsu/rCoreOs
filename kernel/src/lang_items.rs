use crate::sbicall::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        crate::printkln!("Panicked at {} {} => {}", location.file(), location.line(), info.message().unwrap());
    } else {
        crate::printkln!("Panicked: {}", info.message().unwrap());
    }
    shutdown();
}