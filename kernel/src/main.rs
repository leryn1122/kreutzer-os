#![no_std]
#![no_main]
#![feature(allocator_api)]

extern crate alloc;

mod allocator;
mod arch;
mod panic;

#[used]
#[unsafe(link_section = ".multiboot2")]
static MULTIBOOT2_HEADER: multiboot2::Multiboot2Header = multiboot2::Multiboot2Header {
  magic:   multiboot2::Multiboot2Magic::<multiboot2::Multiboot2Header>::MAGIC,
  end_tag: multiboot2::Multiboot2EndTag::TAG,
};

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  loop {
    unsafe {
      core::arch::asm!("hlt", options(nomem, nostack));
    }
  }
}