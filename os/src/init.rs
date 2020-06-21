global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    crate::interrupt::init();
    unsafe {
        #![feature(llvm_asm)]
        llvm_asm!("ebreak"::::"volatile");
    }
    panic!("end of rust_main");
}
