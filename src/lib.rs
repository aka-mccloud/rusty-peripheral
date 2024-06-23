#![no_std]

use register::field::derive::RegisterField;

pub mod cortex_m;
pub mod syscfg;
pub mod rcc;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod spi;

#[derive(RegisterField, Debug, PartialEq)]
pub enum State {
    OFF,
    ON,
}

pub(crate) fn get_peripheral<T>(addr: u32) -> &'static mut T {
    unsafe {
        let ptr: *mut T = addr as *mut T;
        &mut *ptr
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
