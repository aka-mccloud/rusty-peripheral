#![no_std]

use core::ops::{ Deref, DerefMut };

pub mod cortex_m;
pub mod syscfg;
pub mod rcc;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod spi;

pub enum State {
    OFF,
    ON,
}

impl State {
    pub fn from_bits(val: u32) -> State {
        if val == 0 { State::OFF } else { State::ON }
    }

    pub fn into_bits(val: State) -> u32 {
        match val {
            State::OFF => 0,
            State::ON => 1,
        }
    }
}

pub(crate) fn get_peripheral<T>(addr: u32) -> &'static mut T {
    unsafe {
        let ptr: *mut T = addr as *mut T;
        &mut *ptr
    }
}

pub struct RCC {}

impl Deref for RCC {
    type Target = rcc::RegisterBlock;

    fn deref(&self) -> &'static Self::Target {
        unsafe { &*(0x4002_3800u32 as *const _) }
    }
}

impl DerefMut for RCC {
    fn deref_mut(&mut self) -> &'static mut Self::Target {
        unsafe { &mut *(0x4002_3800u32 as *mut _) }
    }
}

#[inline(always)]
pub fn rcc() -> RCC {
    RCC {}
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
