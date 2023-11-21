use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        crate::println!(
            "Paniced at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        crate::println!("Paniced: {}", info.message().unwrap());
    }
    loop {}
}
