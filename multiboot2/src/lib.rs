#![no_std]

use core::marker::PhantomData;

pub const MULTIBOOT2_MAGIC: u32 = 0xe85250d6;
#[cfg(target_arch = "x86_64")]
pub const MULTIBOOT2_ARCH_I386: u32 = 0x0;

#[repr(C)]
pub struct Multiboot2Header {
  pub magic: Multiboot2Magic<Self>,
  pub end_tag: Multiboot2EndTag,
}

#[repr(C)]
pub struct Multiboot2Magic<S> {
  pub magic: u32,
  pub architecture: u32,
  pub header_length: u32,
  pub checksum: u32,
  pub _phantom: PhantomData<S>,
}

impl<S> Multiboot2Magic<S> {
  pub const MAGIC: Self = Self {
    magic: MULTIBOOT2_MAGIC,
    #[cfg(target_arch = "x86_64")]
    architecture: MULTIBOOT2_ARCH_I386,
    header_length: core::mem::size_of::<S>() as u32,
    checksum: (0x100000000i64 -((MULTIBOOT2_MAGIC
      + MULTIBOOT2_ARCH_I386
      + core::mem::size_of::<S>() as u32) as i64))  as u32,
    _phantom: PhantomData,
  };
}

#[repr(C)]
pub struct Multiboot2EndTag {
  r#type: u16,
  flags: u16,
  size: u32,
}

impl Multiboot2EndTag {
  pub const TAG: Multiboot2EndTag = Multiboot2EndTag {
    r#type: 0,
    flags: 0,
    size: 8,
  };
}