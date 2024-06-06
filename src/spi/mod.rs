#![allow(dead_code)]

use core::{ fmt, mem, slice };

use crate::{ get_peripheral, rcc };

use self::register::*;

mod register;

pub struct SPI {
    /// Control Register 1
    cr1: ControlRegister1,

    /// Control Register 2
    cr2: ControlRegister2,

    /// Status Register
    sr: StatusRegister,

    /// Data Register
    dr: DataRegister,

    /// CRC Polynomial Register
    crcpr: CRCPolynomialRegister,

    /// RX CRC Register
    rxcrcr: CRCRegister,

    /// TX CRC Register
    txcrcr: CRCRegister,
}

pub fn spi1() -> &'static mut SPI {
    get_peripheral(0x4001_3000u32)
}

pub fn spi2() -> &'static mut SPI {
    get_peripheral(0x4000_3800u32)
}

pub fn spi3() -> &'static mut SPI {
    get_peripheral(0x4000_3c00u32)
}

pub fn spi4() -> &'static mut SPI {
    get_peripheral(0x4001_3400u32)
}

pub fn spi5() -> &'static mut SPI {
    get_peripheral(0x4001_5000u32)
}

pub fn spi6() -> &'static mut SPI {
    get_peripheral(0x4001_5400u32)
}

impl SPI {
    pub fn enable(&mut self) {
        self.cr1.enable_peripheral(true)
    }

    pub fn disable(&mut self) {
        self.cr1.enable_peripheral(false)
    }

    pub fn is_enabled(&self) -> bool {
        self.cr1.peripheral_is_enabled()
    }

    pub fn init(
        &mut self,
        mode: Mode,
        bus_config: BusConfiguration,
        baud_rate: BaudRate,
        data_format: DataFrameFormat,
        cpol: ClockPolarity,
        cpha: ClockPhase,
        ssm: bool
    ) -> Result<()> {
        self.enable_peripheral_clock();

        self.disable();

        self.cr1.set_mode(mode);
        match bus_config {
            BusConfiguration::FullDuplex => self.cr1.enable_bidirectional_mode(false),
            BusConfiguration::HalfDuplex => self.cr1.enable_bidirectional_mode(true),
            BusConfiguration::SimplexReceiveOnly => {
                self.cr1.enable_bidirectional_mode(false);
                self.cr1.enable_receive_only(true);
            }
        }
        self.cr1.set_baud_rate(baud_rate);
        self.cr1.set_data_frame_format(data_format);
        self.cr1.set_clock_polarity(cpol);
        self.cr1.set_clock_phase(cpha);
        self.cr1.enable_software_slave_management(ssm);

        self.enable();

        Ok(())
    }

    pub fn reset(&self) {
        let ptr = self as *const Self;

        match ptr as u32 {
            0x4001_3000u32 => {
                rcc().apb2rstr.spi1_reset(true);
                rcc().apb2rstr.spi1_reset(false);
            }
            0x4000_3800u32 => {
                rcc().apb1rstr.spi2_reset(true);
                rcc().apb1rstr.spi2_reset(false);
            }
            0x4000_3c00u32 => {
                rcc().apb1rstr.spi3_reset(true);
                rcc().apb1rstr.spi3_reset(false);
            }
            0x4001_3400u32 => {
                rcc().apb2rstr.spi4_reset(true);
                rcc().apb2rstr.spi4_reset(false);
            }
            0x4001_5000u32 => {
                rcc().apb2rstr.spi5_reset(true);
                rcc().apb2rstr.spi5_reset(false);
            }
            0x4001_5400u32 => {
                rcc().apb2rstr.spi6_reset(true);
                rcc().apb2rstr.spi6_reset(false);
            }
            _ => panic!(),
        }
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<()> {
        match self.cr1.get_data_frame_format() {
            DataFrameFormat::Format8Bit => {
                for byte in data {
                    while !self.sr.tx_is_empty() {}
                    self.dr.write_data(*byte as _);
                }
            }
            DataFrameFormat::Format16Bit => {
                let data = unsafe {
                    slice::from_raw_parts(data.as_ptr() as *const _, data.len() / mem::size_of::<u16>())
                };
                for word in data {
                    while !self.sr.tx_is_empty() {}
                    self.dr.write_data(*word);
                }
            }
        }

        Ok(())
    }

    pub fn read_data(&mut self, data: &mut [u8]) -> Result<()> {
        Ok(())
    }

    fn enable_peripheral_clock(&self) {
        let ptr = self as *const Self;

        match ptr as u32 {
            0x4001_3000u32 => {
                rcc().apb2enr.spi1_enable(true);
            }
            0x4000_3800u32 => {
                rcc().apb1enr.spi2_enable(true);
            }
            0x4000_3c00u32 => {
                rcc().apb1enr.spi3_enable(true);
            }
            0x4001_3400u32 => {
                rcc().apb2enr.spi4_enable(true);
            }
            0x4001_5000u32 => {
                rcc().apb2enr.spi5_enable(true);
            }
            0x4001_5400u32 => {
                rcc().apb2enr.spi6_enable(true);
            }
            _ => panic!(),
        }
    }

    fn disable_peripheral_clock(&self) {
        let ptr = self as *const Self;

        match ptr as u32 {
            0x4001_3000u32 => {
                rcc().apb2enr.spi1_enable(false);
            }
            0x4000_3800u32 => {
                rcc().apb1enr.spi2_enable(false);
            }
            0x4000_3c00u32 => {
                rcc().apb1enr.spi3_enable(false);
            }
            0x4001_3400u32 => {
                rcc().apb2enr.spi4_enable(false);
            }
            0x4001_5000u32 => {
                rcc().apb2enr.spi5_enable(false);
            }
            0x4001_5400u32 => {
                rcc().apb2enr.spi6_enable(false);
            }
            _ => panic!(),
        }
    }
}

pub enum BusConfiguration {
    FullDuplex,
    HalfDuplex,
    SimplexReceiveOnly,
}

#[derive(Debug)]
pub enum Error {
    InitError(&'static str),
    BusError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitError(e) => f.write_fmt(format_args!("InitError: {}", e)),
            Error::BusError(e) => f.write_fmt(format_args!("BusError: {}", e)),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
