#![allow(dead_code)]

use core::cell::Cell;

use self::register::*;

pub mod register;

pub struct RegisterBlock {
    /// Clock Control Register
    pub cr: ClockControlRegister,

    /// PLL Configuration Register
    pub pllcfgr: PLLConfigurationRegister,

    /// Clock Configuration Register
    pub cfgr: ClockConfigurationRegister,

    /// Clock Interuupt Register
    pub cir: ClockInterruptRegister,

    /// AHB1 Peripheral Reset Register
    pub ahb1rstr: AHB1PeripheralResetRegister,

    /// AHB2 Peripheral Reset Register
    pub ahb2rstr: AHB2PeripheralResetRegister,

    /// AHB3 Peripheral Reset Register
    pub ahb3rstr: AHB3PeripheralResetRegister,

    __reserved0: u32,

    /// APB1 Peripheral Reset Register
    pub apb1rstr: APB1PeripheralResetRegister,

    /// APB2 Peripheral Reset Register
    pub apb2rstr: APB2PeripheralResetRegister,

    __reserved1: u32,
    __reserved2: u32,

    /// AHB1 Peripheral Clock Register
    pub ahb1enr: AHB1PeripheralClockRegister,

    /// AHB2 Peripheral Clock Register
    pub ahb2enr: AHB2PeripheralClockRegister,

    /// AHB3 Peripheral Clock Register
    pub ahb3enr: AHB3PeripheralClockRegister,

    __reserved3: u32,

    /// APB1 Peripheral Clock Register
    pub apb1enr: APB1PeripheralClockRegister,

    /// APB2 Peripheral Clock Register
    pub apb2enr: APB2PeripheralClockRegister,

    __reserved4: u32,
    __reserved5: u32,

    /// AHB1 Peripheral Clock Enable in Low Power Mode Register
    pub ahb1lpenr: AHB1PeripheralClockLowPowerModeRegister,

    /// AHB2 Peripheral Clock Enable in Low Power Mode Register
    pub ahb2lpenr: AHB2PeripheralClockRegister,

    /// AHB3 Peripheral Clock Enable in Low Power Mode Register
    pub ahb3lpenr: AHB3PeripheralClockRegister,

    __reserved6: u32,

    /// APB1 Peripheral Clock Enable in Low Power Mode Register
    pub apb1lpenr: APB1PeripheralClockRegister,

    /// APB2 Peripheral Clock Enable in Low Power Mode Register
    pub apb2lpenr: APB2PeripheralClockRegister,

    __reserved7: u32,
    __reserved8: u32,

    /// Backup Domain Control Register
    pub bdcr: BackupDomainControlRegister,

    /// Control Control & Status Register
    pub csr: ClockControlAndStatusRegister,

    __reserved9: u32,
    __reserved10: u32,

    /// Spread Spectrum Clock Generation Register
    pub sscgr: SpreadSpectrumClockGenerationRegister,

    /// PLL I2S Configuration Register
    pub plli2scfgr: PLLI2SConfigurationRegister,

    /// PLL SAI Configuration Register
    pub pllsaicfgr: PLLSAIConfigurationRegister,

    /// Dedicated Clock Configuration Register
    pub dckcfgr: DedicatedClockConfigurationRegister,
}

pub fn get_rcc() -> &'static mut RegisterBlock {
    let addr = 0x4002_3800u32;

    unsafe {
        let ptr: *mut RegisterBlock = addr as *mut RegisterBlock;
        &mut *ptr
    }
}

const EXTERNAL_OSC_FREQ: core::cell::Cell<u32> = Cell::new(8_000_000u32);

pub fn set_external_osc_freq(freq: u32) {
    EXTERNAL_OSC_FREQ.set(freq);
}

impl RegisterBlock {
    #[inline]
    pub fn get_sysclock_clock_source(&self) -> SystemClockSource {
        self.cfgr.sysclock_get_used_clock_source()
    }

    #[inline]
    pub fn get_pll_clock_source(&self) -> PLLClockSource {
        self.pllcfgr.pll_get_clock_source()
    }

    #[inline]
    pub fn get_sysclk_freq(&self) -> u32 {
        match self.get_sysclock_clock_source() {
            SystemClockSource::HSI => 16_000_000u32,
            SystemClockSource::HSE => EXTERNAL_OSC_FREQ.get(),
            SystemClockSource::PLL => {
                let freq = match self.get_pll_clock_source() {
                    PLLClockSource::HSI => 16_000_000u32,
                    PLLClockSource::HSE => EXTERNAL_OSC_FREQ.get(),
                };

                let pllp = match self.pllcfgr.pll_get_sysclock_division_factor() {
                    PLLSysClockDivisionFactor::DividedBy2 => 2,
                    PLLSysClockDivisionFactor::DividedBy4 => 4,
                    PLLSysClockDivisionFactor::DividedBy6 => 6,
                    PLLSysClockDivisionFactor::DividedBy8 => 8,
                };

                ((freq / self.pllcfgr.pll_get_division_factor()) *
                    self.pllcfgr.pll_get_mutiplication_factor()) /
                    pllp
            }
        }
    }

    #[inline]
    pub fn get_hclk_freq(&self) -> u32 {
        let ahb_prescaler = match self.cfgr.ahb_get_prescaler() {
            AHBPrescaler::NotDivided => 1,
            AHBPrescaler::DividedBy2 => 2,
            AHBPrescaler::DividedBy4 => 4,
            AHBPrescaler::DividedBy8 => 8,
            AHBPrescaler::DividedBy16 => 16,
            AHBPrescaler::DividedBy64 => 64,
            AHBPrescaler::DividedBy128 => 128,
            AHBPrescaler::DividedBy256 => 256,
            AHBPrescaler::DividedBy512 => 512,
        };

        self.get_sysclk_freq() / ahb_prescaler
    }

    #[inline]
    pub fn get_pclk1_freq(&self) -> u32 {
        let apb1_prescaler = match self.cfgr.apb1_get_prescaler() {
            APBPrescaler::NotDivided => 1,
            APBPrescaler::DividedBy2 => 2,
            APBPrescaler::DividedBy4 => 4,
            APBPrescaler::DividedBy8 => 8,
            APBPrescaler::DividedBy16 => 16,
        };

        self.get_hclk_freq() / apb1_prescaler
    }
}

pub enum SystemClockSource {
    HSI,
    HSE,
    PLL,
}

impl SystemClockSource {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b00 => Self::HSI,
            0b01 => Self::HSE,
            0b10 => Self::PLL,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::HSI => 0b00,
            Self::HSE => 0b01,
            Self::PLL => 0b10,
        }
    }
}

pub enum PLLClockSource {
    HSI,
    HSE,
}

impl PLLClockSource {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::HSI,
            0b1 => Self::HSE,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::HSI => 0b0,
            Self::HSE => 0b1,
        }
    }
}

pub enum MCOClockSource {
    HSI,
    LSE,
    HSE,
    PLL,
}

impl MCOClockSource {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b00 => Self::HSI,
            0b01 => Self::LSE,
            0b10 => Self::HSE,
            0b11 => Self::PLL,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::HSI => 0b00,
            Self::LSE => 0b01,
            Self::HSE => 0b10,
            Self::PLL => 0b11,
        }
    }
}

pub enum I2SClockSource {
    PLLI2S,
    External,
}

impl I2SClockSource {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::PLLI2S,
            0b1 => Self::External,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::PLLI2S => 0b0,
            Self::External => 0b1,
        }
    }
}

pub enum RTCClockSource {
    None,
    LSE,
    LSI,
    HSE,
}

impl RTCClockSource {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b00 => Self::None,
            0b01 => Self::LSE,
            0b10 => Self::LSI,
            0b11 => Self::HSE,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::None => 0b00,
            Self::LSE => 0b01,
            Self::LSI => 0b10,
            Self::HSE => 0b11,
        }
    }
}

pub enum AHBPrescaler {
    NotDivided,
    DividedBy2,
    DividedBy4,
    DividedBy8,
    DividedBy16,
    DividedBy64,
    DividedBy128,
    DividedBy256,
    DividedBy512,
}

impl AHBPrescaler {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b1000 => Self::DividedBy2,
            0b1001 => Self::DividedBy4,
            0b1010 => Self::DividedBy8,
            0b1011 => Self::DividedBy16,
            0b1100 => Self::DividedBy64,
            0b1101 => Self::DividedBy128,
            0b1110 => Self::DividedBy256,
            0b1111 => Self::DividedBy512,
            _ => Self::NotDivided,
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::NotDivided => 0,
            Self::DividedBy2 => 0b1000,
            Self::DividedBy4 => 0b1001,
            Self::DividedBy8 => 0b1010,
            Self::DividedBy16 => 0b1011,
            Self::DividedBy64 => 0b1100,
            Self::DividedBy128 => 0b1101,
            Self::DividedBy256 => 0b1110,
            Self::DividedBy512 => 0b1111,
        }
    }
}

pub enum APBPrescaler {
    NotDivided,
    DividedBy2,
    DividedBy4,
    DividedBy8,
    DividedBy16,
}

impl APBPrescaler {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b100 => Self::DividedBy2,
            0b101 => Self::DividedBy4,
            0b110 => Self::DividedBy8,
            0b111 => Self::DividedBy16,
            _ => Self::NotDivided,
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::NotDivided => 0,
            Self::DividedBy2 => 0b100,
            Self::DividedBy4 => 0b101,
            Self::DividedBy8 => 0b110,
            Self::DividedBy16 => 0b111,
        }
    }
}

pub enum MCOPrescaler {
    NotDivided,
    DividedBy2,
    DividedBy3,
    DividedBy4,
    DividedBy5,
}

impl MCOPrescaler {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b100 => Self::DividedBy2,
            0b101 => Self::DividedBy3,
            0b110 => Self::DividedBy4,
            0b111 => Self::DividedBy5,
            _ => Self::NotDivided,
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::NotDivided => 0,
            Self::DividedBy2 => 0b100,
            Self::DividedBy3 => 0b101,
            Self::DividedBy4 => 0b110,
            Self::DividedBy5 => 0b111,
        }
    }
}

pub enum SpreadSelect {
    CenterSpread,
    DownSpread,
}

impl SpreadSelect {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::CenterSpread,
            0b1 => Self::DownSpread,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::CenterSpread => 0b0,
            Self::DownSpread => 0b1,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum SAIClockSource {
    PLLSAI_PLLSAIDIV,
    PLLI2S_PLLI2SDIV,
    Alternate,
}

impl SAIClockSource {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b00 => Self::PLLSAI_PLLSAIDIV,
            0b01 => Self::PLLI2S_PLLI2SDIV,
            0b10 => Self::Alternate,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::PLLSAI_PLLSAIDIV => 0b00,
            Self::PLLI2S_PLLI2SDIV => 0b01,
            Self::Alternate => 0b10,
        }
    }
}

pub enum TimerPrescaler {
    PRE0,
    PRE1,
}

impl TimerPrescaler {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::PRE0,
            0b1 => Self::PRE1,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        match val {
            Self::PRE0 => 0b0,
            Self::PRE1 => 0b1,
        }
    }
}
