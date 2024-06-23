#![allow(dead_code)]

use core::{ fmt, mem, slice };

use irq::{state_mut, Status};

use crate::{ get_peripheral, rcc::rcc, PeripheralClock };

use self::register::*;

mod register;
mod irq;

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

impl SPI {
    pub fn enable(&mut self) {
        self.cr1.enable_peripheral()
    }

    pub fn disable(&mut self) {
        self.cr1.disable_peripheral()
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
        self.disable();

        self.cr1.set_mode(mode);
        match bus_config {
            BusConfiguration::FullDuplex => self.cr1.disable_bidirectional_mode(),
            BusConfiguration::HalfDuplex => self.cr1.enable_bidirectional_mode(),
            BusConfiguration::SimplexReceiveOnly => {
                self.cr1.disable_bidirectional_mode();
                self.cr1.enable_receive_only();
            }
        }
        self.cr1.set_baud_rate(baud_rate);
        self.cr1.set_data_frame_format(data_format);
        self.cr1.set_clock_polarity(cpol);
        self.cr1.set_clock_phase(cpha);

        if ssm {
            self.cr1.enable_software_slave_management();
        } else {
            self.cr1.disable_software_slave_management();
        }

        self.enable();

        Ok(())
    }

    #[inline]
    pub fn write_word(&mut self, word: u16) -> Result<()> {
        while !self.sr.tx_is_empty() {}
        Ok(self.dr.write_data(word))
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<()> {
        match self.cr1.get_data_frame_format() {
            DataFrameFormat::Format8Bit => {
                for byte in data {
                    self.write_word(*byte as _)?;
                }
            }
            DataFrameFormat::Format16Bit => {
                let data = unsafe {
                    slice::from_raw_parts(
                        data.as_ptr() as *const _,
                        data.len() / mem::size_of::<u16>()
                    )
                };
                for word in data {
                    self.write_word(*word)?;
                }
            }
        }

        Ok(())
    }

    pub fn write_data_begin(&mut self, data: &[u8]) -> Result<()> {
        unsafe {
            let state = state_mut(&self);
            match state.status {
                Status::Ready => Ok(()),
                Status::BusyRx => Err(Error::BusyError("RX in progress")),
                Status::BusyTx => Err(Error::BusyError("TX in progress")),
            }?;

            state.tx_buf = (data.as_ptr(), data.len());
            state.status = Status::BusyTx;
        }

        self.cr2.enable_tx_empty_interrupt();

        Ok(())
    }

    #[inline]
    pub fn read_word(&mut self) -> Result<u16> {
        while !self.sr.rx_is_not_empty() {}
        Ok(self.dr.read_data())
    }

    pub fn read_data(&mut self, data: &mut [u8]) -> Result<()> {
        match self.cr1.get_data_frame_format() {
            DataFrameFormat::Format8Bit => {
                for byte in data {
                    *byte = self.read_word()? as u8;
                }
            }
            DataFrameFormat::Format16Bit => {
                let data = unsafe {
                    slice::from_raw_parts_mut(
                        data.as_ptr() as *mut u16,
                        data.len() / mem::size_of::<u16>()
                    )
                };

                for word in data {
                    *word = self.read_word()?;
                }
            }
        }

        Ok(())
    }

    pub fn read_data_begin(&mut self, data: &mut [u8]) -> Result<()> {
        unsafe {
            let state = state_mut(&self);
            match state.status {
                Status::Ready => Ok(()),
                Status::BusyRx => Err(Error::BusyError("RX in progress")),
                Status::BusyTx => Err(Error::BusyError("TX in progress")),
            }?;
            
            state.rx_buf = (data.as_mut_ptr(), data.len());
            state.status = Status::BusyRx;
        }

        self.cr2.enable_rx_not_empty_interrupt();

        Ok(())
    }
}

impl PeripheralClock for SPI {
    fn reset(&self) {
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

    fn enable_clock(&self) {
        let ptr = self as *const Self;

        match ptr as u32 {
            0x4001_3000u32 => {
                rcc().apb2enr.spi1_enable();
            }
            0x4000_3800u32 => {
                rcc().apb1enr.spi2_enable();
            }
            0x4000_3c00u32 => {
                rcc().apb1enr.spi3_enable();
            }
            0x4001_3400u32 => {
                rcc().apb2enr.spi4_enable();
            }
            0x4001_5000u32 => {
                rcc().apb2enr.spi5_enable();
            }
            0x4001_5400u32 => {
                rcc().apb2enr.spi6_enable();
            }
            _ => panic!(),
        }
    }

    fn disable_clock(&self) {
        let ptr = self as *const Self;

        match ptr as u32 {
            0x4001_3000u32 => {
                rcc().apb2enr.spi1_disable();
            }
            0x4000_3800u32 => {
                rcc().apb1enr.spi2_disable();
            }
            0x4000_3c00u32 => {
                rcc().apb1enr.spi3_disable();
            }
            0x4001_3400u32 => {
                rcc().apb2enr.spi4_disable();
            }
            0x4001_5000u32 => {
                rcc().apb2enr.spi5_disable();
            }
            0x4001_5400u32 => {
                rcc().apb2enr.spi6_disable();
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
    BusyError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitError(e) => f.write_fmt(format_args!("InitError: {}", e)),
            Error::BusError(e) => f.write_fmt(format_args!("BusError: {}", e)),
            Error::BusyError(e) => f.write_fmt(format_args!("BusyError: {}", e)),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

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
