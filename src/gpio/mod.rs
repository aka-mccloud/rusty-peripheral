#![allow(dead_code)]

use ::register::field::RegisterField;

use crate::{ exti::EXTI, rcc::rcc, syscfg::SYSCFG };

use self::{ register::*, port::Port, pin::PinMask };

pub use self::register::{ Mode, OutputType, Pull, Speed };

mod register;
pub mod port;
pub mod pin;

pub fn port(port: Port) -> GPIOPort<'static> {
    let addr = match port {
        Port::A => 0x4002_0000u32,
        Port::B => 0x4002_0400u32,
        Port::C => 0x4002_0800u32,
        Port::D => 0x4002_0c00u32,
        Port::E => 0x4002_1000u32,
        Port::F => 0x4002_1400u32,
        Port::G => 0x4002_1800u32,
        Port::H => 0x4002_1c00u32,
        Port::I => 0x4002_2000u32,
        Port::J => 0x4002_2400u32,
        Port::K => 0x4002_2800u32,
    };

    let p = unsafe {
        let ptr: *mut GPIO = addr as *mut GPIO;
        &mut *ptr
    };

    GPIOPort { regs: p, port }
}

#[derive(Debug, Default)]
pub struct GPIO {
    /// Mode Register
    pub moder: ModeRegister,

    /// Output Type Register
    pub otyper: OutputTypeRegister,

    /// Output Speed Register
    pub ospeedr: OutputSpeedRegister,

    /// Pull-Up/Pull-Down Register
    pub pupdr: PullUpPullDownRegister,

    /// Input Data Register
    pub idr: InputDataRegister,

    /// Output Data Register
    pub odr: OutputDataRegister,

    /// Bit Set/Reset Register
    pub bsrr: BitSetResetRegister,

    /// Lock Configuration Register
    pub lckr: LockConfigurationRegister,

    /// Alternate Function Low Register
    pub afrl: AlternateFunctionLowRegister,

    /// Alternate Function High Register
    pub afrh: AlternateFunctionHighRegister,
}

pub struct GPIOPort<'a> {
    pub(crate) regs: &'a mut GPIO,
    pub(crate) port: Port,
}

impl<'a> GPIOPort<'a> {
    pub fn enable_clock(&self) {
        let enabled = rcc().ahb1enr.gpio_get_enabled();
        rcc().ahb1enr.gpio_enable(enabled | self.port);
    }

    pub fn disable_clock(&self) {
        let enabled = rcc().ahb1enr.gpio_get_enabled();
        rcc().ahb1enr.gpio_enable(enabled & !self.port);
    }

    pub fn reset(&self) {
        rcc().ahb1rstr.gpio_reset(self.port.into());
        rcc().ahb1rstr.gpio_reset_clear();
    }

    pub fn get_input_pins(&self) -> PinMask {
        self.regs.idr.get_pins()
    }

    #[inline]
    pub fn set_pins(&mut self, pins: impl Into<PinMask>) {
        let mask = pins.into();
        self.regs.bsrr.set_pins(mask.into());
    }

    #[inline]
    pub fn reset_pins(&mut self, pins: impl Into<PinMask>) {
        let mask = pins.into();
        self.regs.bsrr.reset_pins(mask.into());
    }

    #[inline]
    pub fn toggle_pins(&mut self, pins: impl Into<PinMask>) {
        let mask = pins.into();
        let odr = self.regs.odr.get_pins();
        let val = ((odr & mask).into_bits() << 16) | (!odr & mask).into_bits();
        self.regs.bsrr.set(val);
    }

    #[inline]
    pub fn init_pins(&mut self, pins: impl Into<PinMask>, conf: PinConfig) {
        let mask = pins.into();
        // to populate bits use multiply by pattern
        // e.g. val is 2 bits value
        // val * 01_01_01_01 = val_val_val_val
        // e.g. val is 3 bits value
        // val * 001_001_001_001 = val_val_val_val
        // and so on, and so forth

        match conf {
            PinConfig::Input(ospeed, pupd, interrupt) => {
                let mut moder = self.regs.moder.get();
                let mut ospeedr = self.regs.ospeedr.get();
                let mut pupdr = self.regs.pupdr.get();

                let mask2 = mask.mask_2bit();

                moder &= !mask2;
                ospeedr &= !mask2;
                ospeedr |= mask2 & (ospeed.into_bits() * 0x55555555u32);
                pupdr &= !mask2;
                pupdr |= mask2 & (pupd.into_bits() * 0x55555555u32);

                self.regs.moder.set(moder);
                self.regs.ospeedr.set(ospeedr);
                self.regs.pupdr.set(pupdr);

                match interrupt {
                    InterruptType::None => (),
                    InterruptType::RisingEdge => {
                        let syscfg = SYSCFG::get();
                        syscfg.set_external_interrupt_source(self.port, mask);

                        let exti = EXTI::get();
                        exti.set_rising_trigger_lines(mask);
                        exti.reset_falling_trigger_lines(mask);

                        exti.unmask_interrupts_lines(mask);
                    }
                    InterruptType::FallingEdge => {
                        let syscfg = SYSCFG::get();
                        syscfg.set_external_interrupt_source(self.port, mask);

                        let exti = EXTI::get();
                        exti.reset_rising_trigger_lines(mask);
                        exti.set_falling_trigger_lines(mask);

                        exti.unmask_interrupts_lines(mask);
                    }
                    InterruptType::RisingFallingEdge => {
                        let syscfg = SYSCFG::get();
                        syscfg.set_external_interrupt_source(self.port, mask);

                        let exti = EXTI::get();
                        exti.set_rising_trigger_lines(mask);
                        exti.set_falling_trigger_lines(mask);

                        exti.unmask_interrupts_lines(mask);
                    }
                }
            }
            PinConfig::Output(otype, ospeed, pull) => {
                let mut moder = self.regs.moder.get();
                let mut otyper = self.regs.otyper.get();
                let mut ospeedr = self.regs.ospeedr.get();
                let mut pupdr = self.regs.pupdr.get();

                let mask1 = mask.mask_1bit();
                let mask2 = mask.mask_2bit();
                moder &= !mask2;
                moder |= mask2 & (Mode::into_bits(Mode::Output) * 0x55555555u32);
                otyper &= !mask1;
                otyper |= mask1 & (OutputType::into_bits(otype) * 0xffffu32);
                ospeedr &= !mask2;
                ospeedr |= mask2 & (Speed::into_bits(ospeed) * 0x55555555u32);
                pupdr &= !mask2;
                pupdr |= mask2 & (Pull::into_bits(pull) * 0x55555555u32);

                self.regs.moder.set(moder);
                self.regs.otyper.set(otyper);
                self.regs.ospeedr.set(ospeedr);
                self.regs.pupdr.set(pupdr);
            }
            PinConfig::Alternate(af, otype, ospeed, pupd) => {
                let mut moder = self.regs.moder.get();
                let mut otyper = self.regs.otyper.get();
                let mut ospeedr = self.regs.ospeedr.get();
                let mut pupdr = self.regs.pupdr.get();
                let mut afrl = self.regs.afrl.get();
                let mut afrh = self.regs.afrh.get();

                let mask1 = mask.mask_1bit();
                let mask2 = mask.mask_2bit();
                let mask4 = mask.mask_4bit();

                moder &= !mask2;
                moder |= mask2 & (Mode::into_bits(Mode::Alternate) * 0x55555555u32);
                otyper &= !mask1;
                otyper |= mask1 & (OutputType::into_bits(otype) * 0xffffu32);
                ospeedr &= !mask2;
                ospeedr |= mask2 & (Speed::into_bits(ospeed) * 0x55555555u32);
                pupdr &= !mask2;
                pupdr |= mask2 & (Pull::into_bits(pupd) * 0x55555555u32);
                afrh &= !mask4.0;
                afrh |= mask4.0 & ((af as u32) * 0x11111111u32);
                afrl &= !mask4.1;
                afrl |= mask4.1 & ((af as u32) * 0x11111111u32);

                self.regs.moder.set(moder);
                self.regs.otyper.set(otyper);
                self.regs.ospeedr.set(ospeedr);
                self.regs.pupdr.set(pupdr);
                self.regs.afrl.set(afrl);
                self.regs.afrh.set(afrh);
            }
            PinConfig::Analog => {
                let mut moder = self.regs.moder.get();

                let mask2 = mask.mask_2bit();

                moder &= !mask2;
                moder |= mask2 & (Mode::into_bits(Mode::Analog) * 0x55555555u32);

                self.regs.moder.set(moder);
            }
        }
    }
}

pub enum InterruptType {
    None,
    RisingEdge,
    FallingEdge,
    RisingFallingEdge,
}

pub enum PinConfig {
    Input(Speed, Pull, InterruptType),
    Output(OutputType, Speed, Pull),
    Alternate(u8, OutputType, Speed, Pull),
    Analog,
}

#[cfg(test)]
mod tests {
    use super::pin::Pin;

    use super::*;

    #[test]
    fn test_pin_masks() {
        let mask =
            Pin::PIN0 |
            Pin::PIN2 |
            Pin::PIN4 |
            Pin::PIN6 |
            Pin::PIN8 |
            Pin::PIN10 |
            Pin::PIN12 |
            Pin::PIN14;

        assert_eq!(0x00005555u32, mask.mask_1bit());
        assert_eq!(0x33333333u32, mask.mask_2bit());
        assert_eq!((0x0f0f0f0fu32, 0x0f0f0f0fu32), mask.mask_4bit());

        let mask =
            Pin::PIN1 |
            Pin::PIN3 |
            Pin::PIN5 |
            Pin::PIN7 |
            Pin::PIN9 |
            Pin::PIN11 |
            Pin::PIN13 |
            Pin::PIN15;

        assert_eq!(0x0000aaaau32, mask.mask_1bit());
        assert_eq!(0xccccccccu32, mask.mask_2bit());
        assert_eq!((0xf0f0f0f0u32, 0xf0f0f0f0u32), mask.mask_4bit());
    }

    #[test]
    fn init_two_output_pins() {
        let mut reg = GPIO::default();
        let mut gpio = GPIOPort {
            regs: &mut reg,
            port: Port::A,
        };

        assert_eq!(gpio.regs.moder.get(), 0u32);
        assert_eq!(gpio.regs.otyper.get(), 0u32);
        assert_eq!(gpio.regs.ospeedr.get(), 0u32);
        assert_eq!(gpio.regs.pupdr.get(), 0u32);

        gpio.init_pins(
            Pin::PIN0 | Pin::PIN2,
            PinConfig::Output(OutputType::PushPull, Speed::High, Pull::Up)
        );

        assert_eq!(gpio.regs.moder.pin0_get_mode(), Mode::Output);
        assert_eq!(gpio.regs.moder.pin1_get_mode(), Mode::Input);
        assert_eq!(gpio.regs.moder.pin2_get_mode(), Mode::Output);
        assert_eq!(gpio.regs.moder.pin3_get_mode(), Mode::Input);
        assert_eq!(gpio.regs.moder.get(), 0b0001_0001);

        assert_eq!(gpio.regs.otyper.pin0_get_output_type(), OutputType::PushPull);
        assert_eq!(gpio.regs.otyper.pin1_get_output_type(), OutputType::PushPull);
        assert_eq!(gpio.regs.otyper.pin2_get_output_type(), OutputType::PushPull);
        assert_eq!(gpio.regs.otyper.pin3_get_output_type(), OutputType::PushPull);
        assert_eq!(gpio.regs.otyper.get(), 0b0000_0000);

        assert_eq!(gpio.regs.ospeedr.pin0_get_output_speed(), Speed::High);
        assert_eq!(gpio.regs.ospeedr.pin1_get_output_speed(), Speed::Low);
        assert_eq!(gpio.regs.ospeedr.pin2_get_output_speed(), Speed::High);
        assert_eq!(gpio.regs.ospeedr.pin3_get_output_speed(), Speed::Low);
        assert_eq!(gpio.regs.ospeedr.get(), 0b0010_0010);

        assert_eq!(gpio.regs.pupdr.pin0_get_pupd(), Pull::Up);
        assert_eq!(gpio.regs.pupdr.pin1_get_pupd(), Pull::None);
        assert_eq!(gpio.regs.pupdr.pin2_get_pupd(), Pull::Up);
        assert_eq!(gpio.regs.pupdr.pin3_get_pupd(), Pull::None);
        assert_eq!(gpio.regs.pupdr.get(), 0b0001_0001);
    }

    #[test]
    fn init_two_alternate_pins() {
        let mut regs = GPIO::default();
        let mut gpio = GPIOPort {
            regs: &mut regs,
            port: Port::B,
        };

        assert_eq!(gpio.regs.moder.get(), 0u32);
        assert_eq!(gpio.regs.otyper.get(), 0u32);
        assert_eq!(gpio.regs.ospeedr.get(), 0u32);
        assert_eq!(gpio.regs.pupdr.get(), 0u32);

        gpio.init_pins(
            Pin::PIN10 | Pin::PIN12,
            PinConfig::Alternate(6, OutputType::OpenDrain, Speed::VeryHigh, Pull::Up)
        );

        assert_eq!(gpio.regs.moder.pin10_get_mode(), Mode::Alternate);
        assert_eq!(gpio.regs.moder.pin11_get_mode(), Mode::Input);
        assert_eq!(gpio.regs.moder.pin12_get_mode(), Mode::Alternate);
        assert_eq!(gpio.regs.moder.pin13_get_mode(), Mode::Input);
        assert_eq!(gpio.regs.moder.get(), 0b00000010_00100000_00000000_00000000);

        assert_eq!(gpio.regs.otyper.pin10_get_output_type(), OutputType::OpenDrain);
        assert_eq!(gpio.regs.otyper.pin11_get_output_type(), OutputType::PushPull);
        assert_eq!(gpio.regs.otyper.pin12_get_output_type(), OutputType::OpenDrain);
        assert_eq!(gpio.regs.otyper.pin13_get_output_type(), OutputType::PushPull);
        assert_eq!(gpio.regs.otyper.get(), 0b00010100_00000000);

        assert_eq!(gpio.regs.ospeedr.pin10_get_output_speed(), Speed::VeryHigh);
        assert_eq!(gpio.regs.ospeedr.pin11_get_output_speed(), Speed::Low);
        assert_eq!(gpio.regs.ospeedr.pin12_get_output_speed(), Speed::VeryHigh);
        assert_eq!(gpio.regs.ospeedr.pin13_get_output_speed(), Speed::Low);
        assert_eq!(gpio.regs.ospeedr.get(), 0b00000011_00110000_00000000_00000000);

        assert_eq!(gpio.regs.pupdr.pin10_get_pupd(), Pull::Up);
        assert_eq!(gpio.regs.pupdr.pin11_get_pupd(), Pull::None);
        assert_eq!(gpio.regs.pupdr.pin12_get_pupd(), Pull::Up);
        assert_eq!(gpio.regs.pupdr.pin13_get_pupd(), Pull::None);
        assert_eq!(gpio.regs.pupdr.get(), 0b00000001_00010000_00000000_00000000);

        assert_eq!(gpio.regs.afrh.pin10_get_alternate_function(), 6);
        assert_eq!(gpio.regs.afrh.pin11_get_alternate_function(), 0);
        assert_eq!(gpio.regs.afrh.pin12_get_alternate_function(), 6);
        assert_eq!(gpio.regs.afrh.pin13_get_alternate_function(), 0);
        assert_eq!(gpio.regs.afrh.get(), 0b00000000_00000110_00000110_00000000);
    }
}
