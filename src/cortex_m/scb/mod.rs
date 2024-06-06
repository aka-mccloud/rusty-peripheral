#![allow(dead_code)]

use self::register::*;

mod register;

pub struct RegisterBlock {
    /// CPUID Base Register
    cpuid: CPUIDBaseRegister,

    /// Interrupt Control and State Register
    icsr: InterruptControlAndStateRegister,
}

impl RegisterBlock {
    pub fn get_active_interrupt_number(&self) -> i16 {
        self.icsr.get_active_vector() as i16 - 16
    }
}
