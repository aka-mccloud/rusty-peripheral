#![allow(dead_code)]

use core::cell::Cell;

use crate::get_peripheral;

use self::register::*;

pub use register::{
    AHBPrescaler,
    APBPrescaler,
    I2SClockSource,
    MCOClockSource,
    MCOPrescaler,
    PLLClockSource,
    PLLSysClockDivisionFactor,
    RTCClockSource,
    SAIClockSource,
    SpreadSelect,
    SystemClockSource,
    TimerPrescaler,
};

mod register;

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

pub fn rcc() -> &'static mut RegisterBlock {
    get_peripheral(0x4002_3800u32)
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
