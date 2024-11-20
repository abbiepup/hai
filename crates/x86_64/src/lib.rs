#![no_std]
#![cfg(target_arch = "x86_64")]
#![feature(abi_x86_interrupt)]

pub mod address;
pub mod interrupt;
pub mod paging;
pub mod register;
