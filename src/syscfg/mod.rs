#![allow(dead_code)]

use crate::gpio::{ pin::PinMask, port::Port };

use self::register::*;

pub use self::register::{
    MemoryMappingMode,
    FlashBankMode,
    FMCMemorySwappingMode,
    EthernetPHYInterface,
};

mod register;

#[derive(Debug, Default)]
pub struct SYSCFG {
    /// Memory Remap Register
    pub memrmp: MemoryRemapRegister,

    /// Peripheral Mode Configuration Register
    pub pmc: PeripheralModeConfigurationRegister,

    /// External Interrupt configuration Registers
    pub exticr: [ExternalInterruptConfigurationRegister; 4],

    /// Compensation Cell Control Register
    pub cmpcr: CompensationCellControlRegister,
}

impl SYSCFG {
    pub fn get() -> &'static mut Self {
        let addr = 0x4001_3800u32;

        unsafe {
            let ptr: *mut Self = addr as *mut Self;
            &mut *ptr
        }
    }

    pub fn set_external_interrupt_source(&mut self, port: Port, pins: impl Into<PinMask>) {
        let port_num = match port {
            Port::A => 0b0000u16,
            Port::B => 0b0001u16,
            Port::C => 0b0010u16,
            Port::D => 0b0011u16,
            Port::E => 0b0100u16,
            Port::F => 0b0101u16,
            Port::G => 0b0110u16,
            Port::H => 0b0111u16,
            Port::I => 0b1000u16,
            Port::J => 0b1001u16,
            Port::K => 0b1010u16,
        };

        let pm4 = pins.into().mask_4bit();

        let mut val = self.exticr[0].get_exti_config();
        val = (val & !(pm4.1 as u16)) | ((port_num * 0x1111u16) & (pm4.1 as u16));
        self.exticr[0].set_exti_config(val);

        let mut val = self.exticr[1].get_exti_config();
        val = (val & !((pm4.1 >> 16) as u16)) | ((port_num * 0x1111u16) & ((pm4.1 >> 16) as u16));
        self.exticr[1].set_exti_config(val);

        let mut val = self.exticr[2].get_exti_config();
        val = (val & !(pm4.0 as u16)) | ((port_num * 0x1111u16) & (pm4.0 as u16));
        self.exticr[2].set_exti_config(val);

        let mut val = self.exticr[3].get_exti_config();
        val = (val & !((pm4.0 >> 16) as u16)) | ((port_num * 0x1111u16) & ((pm4.0 >> 16) as u16));
        self.exticr[3].set_exti_config(val);
    }
}

#[cfg(test)]
mod tests {
    use crate::gpio::pin::Pin;

    use super::*;

    #[test]
    fn test_set_external_interrupt_source() {
        let mut syscfg = SYSCFG::default();

        syscfg.set_external_interrupt_source(
            Port::G,
            Pin::PIN1 | Pin::PIN4 | Pin::PIN10 | Pin::PIN15
        );

        assert_eq!(syscfg.exticr[0].get_exti_config(), 0b0000_0000_0110_0000);
        assert_eq!(syscfg.exticr[1].get_exti_config(), 0b0000_0000_0000_0110);
        assert_eq!(syscfg.exticr[2].get_exti_config(), 0b0000_0110_0000_0000);
        assert_eq!(syscfg.exticr[3].get_exti_config(), 0b0110_0000_0000_0000);

        let mut syscfg = SYSCFG::default();

        syscfg.set_external_interrupt_source(Port::D, Pin::PIN0);
        syscfg.set_external_interrupt_source(Port::E, Pin::PIN1);
        syscfg.set_external_interrupt_source(Port::F, Pin::PIN2);
        syscfg.set_external_interrupt_source(Port::G, Pin::PIN3);

        assert_eq!(syscfg.exticr[0].get_exti_config(), 0b0110_0101_0100_0011);
        assert_eq!(syscfg.exticr[1].get_exti_config(), 0b0000_0000_0000_0000);
        assert_eq!(syscfg.exticr[2].get_exti_config(), 0b0000_0000_0000_0000);
        assert_eq!(syscfg.exticr[3].get_exti_config(), 0b0000_0000_0000_0000);

        syscfg.set_external_interrupt_source(Port::A, Pin::PIN1);
        assert_eq!(syscfg.exticr[0].get_exti_config(), 0b0110_0101_0000_0011);
    }
}
