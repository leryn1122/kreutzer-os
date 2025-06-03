use core::panic::PanicInfo;

/// Panic handler must be implemented manually if using `no_std`.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {
    unsafe {
      core::arch::asm!("hlt", options(nomem, nostack));
    }
  }
}