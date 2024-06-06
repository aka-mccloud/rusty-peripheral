use register::register;

use super::{ Mode, OutputType, Speed, Pull, PinMask };

#[register(u32)]
#[derive(Debug, Default)]
pub struct ModeRegister {
    #[bits(2, rw, get = pin0_get_mode, set = pin0_set_mode)]
    pub MODER0: Mode,

    #[bits(2, rw, get = pin1_get_mode, set = pin1_set_mode)]
    pub MODER1: Mode,

    #[bits(2, rw, get = pin2_get_mode, set = pin2_set_mode)]
    pub MODER2: Mode,

    #[bits(2, rw, get = pin3_get_mode, set = pin3_set_mode)]
    pub MODER3: Mode,

    #[bits(2, rw, get = pin4_get_mode, set = pin4_set_mode)]
    pub MODER4: Mode,

    #[bits(2, rw, get = pin5_get_mode, set = pin5_set_mode)]
    pub MODER5: Mode,

    #[bits(2, rw, get = pin6_get_mode, set = pin6_set_mode)]
    pub MODER6: Mode,

    #[bits(2, rw, get = pin7_get_mode, set = pin7_set_mode)]
    pub MODER7: Mode,

    #[bits(2, rw, get = pin8_get_mode, set = pin8_set_mode)]
    pub MODER8: Mode,

    #[bits(2, rw, get = pin9_get_mode, set = pin9_set_mode)]
    pub MODER9: Mode,

    #[bits(2, rw, get = pin10_get_mode, set = pin10_set_mode)]
    pub MODER10: Mode,

    #[bits(2, rw, get = pin11_get_mode, set = pin11_set_mode)]
    pub MODER11: Mode,

    #[bits(2, rw, get = pin12_get_mode, set = pin12_set_mode)]
    pub MODER12: Mode,

    #[bits(2, rw, get = pin13_get_mode, set = pin13_set_mode)]
    pub MODER13: Mode,

    #[bits(2, rw, get = pin14_get_mode, set = pin14_set_mode)]
    pub MODER14: Mode,

    #[bits(2, rw, get = pin15_get_mode, set = pin15_set_mode)]
    pub MODER15: Mode,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct OutputTypeRegister {
    #[bits(1, rw, get = pin0_get_output_type, set = pin0_set_output_type)]
    pub OT0: OutputType,

    #[bits(1, rw, get = pin1_get_output_type, set = pin1_set_output_type)]
    pub OT1: OutputType,

    #[bits(1, rw, get = pin2_get_output_type, set = pin2_set_output_type)]
    pub OT2: OutputType,

    #[bits(1, rw, get = pin3_get_output_type, set = pin3_set_output_type)]
    pub OT3: OutputType,

    #[bits(1, rw, get = pin4_get_output_type, set = pin4_set_output_type)]
    pub OT4: OutputType,

    #[bits(1, rw, get = pin5_get_output_type, set = pin5_set_output_type)]
    pub OT5: OutputType,

    #[bits(1, rw, get = pin6_get_output_type, set = pin6_set_output_type)]
    pub OT6: OutputType,

    #[bits(1, rw, get = pin7_get_output_type, set = pin7_set_output_type)]
    pub OT7: OutputType,

    #[bits(1, rw, get = pin8_get_output_type, set = pin8_set_output_type)]
    pub OT8: OutputType,

    #[bits(1, rw, get = pin9_get_output_type, set = pin9_set_output_type)]
    pub OT9: OutputType,

    #[bits(1, rw, get = pin10_get_output_type, set = pin10_set_output_type)]
    pub OT10: OutputType,

    #[bits(1, rw, get = pin11_get_output_type, set = pin11_set_output_type)]
    pub OT11: OutputType,

    #[bits(1, rw, get = pin12_get_output_type, set = pin12_set_output_type)]
    pub OT12: OutputType,

    #[bits(1, rw, get = pin13_get_output_type, set = pin13_set_output_type)]
    pub OT13: OutputType,

    #[bits(1, rw, get = pin14_get_output_type, set = pin14_set_output_type)]
    pub OT14: OutputType,

    #[bits(1, rw, get = pin15_get_output_type, set = pin15_set_output_type)]
    pub OT15: OutputType,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct OutputSpeedRegister {
    #[bits(2, rw, get = pin0_get_output_speed, set = pin0_set_output_speed)]
    pub OSPEEDR0: Speed,

    #[bits(2, rw, get = pin1_get_output_speed, set = pin1_set_output_speed)]
    pub OSPEEDR1: Speed,

    #[bits(2, rw, get = pin2_get_output_speed, set = pin2_set_output_speed)]
    pub OSPEEDR2: Speed,

    #[bits(2, rw, get = pin3_get_output_speed, set = pin3_set_output_speed)]
    pub OSPEEDR3: Speed,

    #[bits(2, rw, get = pin4_get_output_speed, set = pin4_set_output_speed)]
    pub OSPEEDR4: Speed,

    #[bits(2, rw, get = pin5_get_output_speed, set = pin5_set_output_speed)]
    pub OSPEEDR5: Speed,

    #[bits(2, rw, get = pin6_get_output_speed, set = pin6_set_output_speed)]
    pub OSPEEDR6: Speed,

    #[bits(2, rw, get = pin7_get_output_speed, set = pin7_set_output_speed)]
    pub OSPEEDR7: Speed,

    #[bits(2, rw, get = pin8_get_output_speed, set = pin8_set_output_speed)]
    pub OSPEEDR8: Speed,

    #[bits(2, rw, get = pin9_get_output_speed, set = pin9_set_output_speed)]
    pub OSPEEDR9: Speed,

    #[bits(2, rw, get = pin10_get_output_speed, set = pin10_set_output_speed)]
    pub OSPEEDR10: Speed,

    #[bits(2, rw, get = pin11_get_output_speed, set = pin11_set_output_speed)]
    pub OSPEEDR11: Speed,

    #[bits(2, rw, get = pin12_get_output_speed, set = pin12_set_output_speed)]
    pub OSPEEDR12: Speed,

    #[bits(2, rw, get = pin13_get_output_speed, set = pin13_set_output_speed)]
    pub OSPEEDR13: Speed,

    #[bits(2, rw, get = pin14_get_output_speed, set = pin14_set_output_speed)]
    pub OSPEEDR14: Speed,

    #[bits(2, rw, get = pin15_get_output_speed, set = pin15_set_output_speed)]
    pub OSPEEDR15: Speed,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct PullUpPullDownRegister {
    #[bits(2, rw, get = pin0_get_pupd, set = pin0_set_pupd)]
    pub PUPDR0: Pull,

    #[bits(2, rw, get = pin1_get_pupd, set = pin1_set_pupd)]
    pub PUPDR1: Pull,

    #[bits(2, rw, get = pin2_get_pupd, set = pin2_set_pupd)]
    pub PUPDR2: Pull,

    #[bits(2, rw, get = pin3_get_pupd, set = pin3_set_pupd)]
    pub PUPDR3: Pull,

    #[bits(2, rw, get = pin4_get_pupd, set = pin4_set_pupd)]
    pub PUPDR4: Pull,

    #[bits(2, rw, get = pin5_get_pupd, set = pin5_set_pupd)]
    pub PUPDR5: Pull,

    #[bits(2, rw, get = pin6_get_pupd, set = pin6_set_pupd)]
    pub PUPDR6: Pull,

    #[bits(2, rw, get = pin7_get_pupd, set = pin7_set_pupd)]
    pub PUPDR7: Pull,

    #[bits(2, rw, get = pin8_get_pupd, set = pin8_set_pupd)]
    pub PUPDR8: Pull,

    #[bits(2, rw, get = pin9_get_pupd, set = pin9_set_pupd)]
    pub PUPDR9: Pull,

    #[bits(2, rw, get = pin10_get_pupd, set = pin10_set_pupd)]
    pub PUPDR10: Pull,

    #[bits(2, rw, get = pin11_get_pupd, set = pin11_set_pupd)]
    pub PUPDR11: Pull,

    #[bits(2, rw, get = pin12_get_pupd, set = pin12_set_pupd)]
    pub PUPDR12: Pull,

    #[bits(2, rw, get = pin13_get_pupd, set = pin13_set_pupd)]
    pub PUPDR13: Pull,

    #[bits(2, rw, get = pin14_get_pupd, set = pin14_set_pupd)]
    pub PUPDR14: Pull,

    #[bits(2, rw, get = pin15_get_pupd, set = pin15_set_pupd)]
    pub PUPDR15: Pull,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct InputDataRegister {
    #[bits(16, ro, get = get_pins)]
    pub IDR: PinMask,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct OutputDataRegister {
    #[bits(16, rw, get = get_pins, set = set_pins)]
    pub ODR: PinMask,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct BitSetResetRegister {
    #[bits(16, wo, set = set_pins)]
    pub BS: u16,

    #[bits(16, wo, set = reset_pins)]
    pub BR: u16,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct LockConfigurationRegister {
    #[bits(1, rw, get = pin0_is_locked, set = pin0_lock)]
    pub LCK0: bool,

    #[bits(1, rw, get = pin1_is_locked, set = pin1_lock)]
    pub LCK1: bool,

    #[bits(1, rw, get = pin2_is_locked, set = pin2_lock)]
    pub LCK2: bool,

    #[bits(1, rw, get = pin3_is_locked, set = pin3_lock)]
    pub LCK3: bool,

    #[bits(1, rw, get = pin4_is_locked, set = pin4_lock)]
    pub LCK4: bool,

    #[bits(1, rw, get = pin5_is_locked, set = pin5_lock)]
    pub LCK5: bool,

    #[bits(1, rw, get = pin6_is_locked, set = pin6_lock)]
    pub LCK6: bool,

    #[bits(1, rw, get = pin7_is_locked, set = pin7_lock)]
    pub LCK7: bool,

    #[bits(1, rw, get = pin8_is_locked, set = pin8_lock)]
    pub LCK8: bool,

    #[bits(1, rw, get = pin9_is_locked, set = pin9_lock)]
    pub LCK9: bool,

    #[bits(1, rw, get = pin10_is_locked, set = pin10_lock)]
    pub LCK10: bool,

    #[bits(1, rw, get = pin11_is_locked, set = pin11_lock)]
    pub LCK11: bool,

    #[bits(1, rw, get = pin12_is_locked, set = pin12_lock)]
    pub LCK12: bool,

    #[bits(1, rw, get = pin13_is_locked, set = pin13_lock)]
    pub LCK13: bool,

    #[bits(1, rw, get = pin14_is_locked, set = pin14_lock)]
    pub LCK14: bool,

    #[bits(1, rw, get = pin15_is_locked, set = pin15_lock)]
    pub LCK15: bool,

    #[bits(1, rw, get = lock_key_is_locked, set = lock_key_lock)]
    pub LCKK: bool,

    #[bits(15)]
    __: u32,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct AlternateFunctionLowRegister {
    #[bits(4, rw, get = pin0_get_alternate_function, set = pin0_set_alternate_function)]
    pub AFRL0: u8,

    #[bits(4, rw, get = pin1_get_alternate_function, set = pin1_set_alternate_function)]
    pub AFRL1: u8,

    #[bits(4, rw, get = pin2_get_alternate_function, set = pin2_set_alternate_function)]
    pub AFRL2: u8,

    #[bits(4, rw, get = pin3_get_alternate_function, set = pin3_set_alternate_function)]
    pub AFRL3: u8,

    #[bits(4, rw, get = pin4_get_alternate_function, set = pin4_set_alternate_function)]
    pub AFRL4: u8,

    #[bits(4, rw, get = pin5_get_alternate_function, set = pin5_set_alternate_function)]
    pub AFRL5: u8,

    #[bits(4, rw, get = pin6_get_alternate_function, set = pin6_set_alternate_function)]
    pub AFRL6: u8,

    #[bits(4, rw, get = pin7_get_alternate_function, set = pin7_set_alternate_function)]
    pub AFRL7: u8,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct AlternateFunctionHighRegister {
    #[bits(4, rw, get = pin8_get_alternate_function, set = pin8_set_alternate_function)]
    pub AFRH8: u8,

    #[bits(4, rw, get = pin9_get_alternate_function, set = pin9_set_alternate_function)]
    pub AFRH9: u8,

    #[bits(4, rw, get = pin10_get_alternate_function, set = pin10_set_alternate_function)]
    pub AFRH10: u8,

    #[bits(4, rw, get = pin11_get_alternate_function, set = pin11_set_alternate_function)]
    pub AFRH11: u8,

    #[bits(4, rw, get = pin12_get_alternate_function, set = pin12_set_alternate_function)]
    pub AFRH12: u8,

    #[bits(4, rw, get = pin13_get_alternate_function, set = pin13_set_alternate_function)]
    pub AFRH13: u8,

    #[bits(4, rw, get = pin14_get_alternate_function, set = pin14_set_alternate_function)]
    pub AFRH14: u8,

    #[bits(4, rw, get = pin15_get_alternate_function, set = pin15_set_alternate_function)]
    pub AFRH15: u8,
}
