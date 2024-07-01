#![no_std]

use register::field::derive::RegisterField;

pub mod cortex_m;
pub mod syscfg;
pub mod rcc;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod spi;
pub mod usart;

pub trait PeripheralClock {
    fn reset(&self);
    fn enable_clock(&self);
    fn disable_clock(&self);
}

#[derive(RegisterField, Debug, PartialEq)]
pub enum State {
    OFF,
    ON,
}

#[inline(always)]
pub(crate) fn peripheral<T>(addr: usize) -> &'static mut T {
    unsafe {
        let ptr = addr as *mut T;
        &mut *ptr
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
