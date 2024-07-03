#![allow(dead_code)]

use core::fmt::{ self };

use irq::{state_mut, Status};
use ::register::field::derive::RegisterField;

use crate::{ peripheral, rcc::rcc, PeripheralClock };

use self::register::*;

mod register;
mod irq;

pub struct I2C {
    /// Control Register 1
    cr1: ControlRegister1,

    /// Control Register 2
    cr2: ControlRegister2,

    /// Own Address Register 1
    oar1: OwnAddressRegister1,

    /// Own Address Register 2
    oar2: OwnAddressRegister2,

    /// Data Register
    dr: DataRegister,

    /// Status Register 1
    sr1: StatusRegister1,

    /// Status Register 2
    sr2: StatusRegister2,

    /// Clock Control Register
    ccr: ClockControlRegister,

    /// Maximum Rise Time Register
    trise: RiseTimeRegister,

    /// Filter Register
    fltr: FilterRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InitError(&'static str),
    BusError,
    NoSlaveAddress(u8),
    BusyError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitError(e) => f.write_fmt(format_args!("InitError: {}", e)),
            Error::BusError => f.write_str("Bus Error"),
            Error::NoSlaveAddress(a) => f.write_fmt(format_args!("No Slave with address: {}", a)),
            Error::BusyError(e) => f.write_fmt(format_args!("BusyError: {}", e)),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl I2C {
    pub fn enable(&mut self) {
        self.cr1.enable_peripheral()
    }

    pub fn disable(&mut self) {
        self.cr1.disable_peripheral()
    }

    pub fn is_enabled(&self) -> bool {
        self.cr1.peripheral_is_enabled()
    }

    pub fn init(&mut self, mode: I2CMode, speed_mode: SpeedMode, scl_freq: u32) -> Result<()> {
        self.disable();

        if scl_freq > 100_000 && speed_mode == SpeedMode::StandardMode {
            return Err(Error::InitError("Frequency is too high"));
        }

        if scl_freq <= 100_000 && speed_mode != SpeedMode::StandardMode {
            return Err(Error::InitError("Frequency is too low"));
        }

        // calculate CCR
        let f_pclk1 = rcc().pclk1_freq();

        let (ccr, trise) = match speed_mode {
            SpeedMode::StandardMode => (f_pclk1 / scl_freq / 2, f_pclk1 / 1_000_000 + 1),
            SpeedMode::FastModeDuty2 => (f_pclk1 / scl_freq / 3, (f_pclk1 * 3) / 10_000_000 + 1),
            SpeedMode::FastModeDuty16_9 =>
                (f_pclk1 / scl_freq / 25, (f_pclk1 * 3) / 10_000_000 + 1),
        };

        self.cr1.set_ack(true);
        self.cr2.set_peripheral_clock_frequency(f_pclk1 / 1_000_000);
        self.ccr.set_speed_mode(speed_mode);
        self.ccr.set_ccr(ccr);
        self.trise.set_rise_time(trise);

        self.oar1.set_as_one(true);
        match mode {
            I2CMode::Master => (),
            I2CMode::Slave { addr1, addr2 } => {
                self.oar1.set_address(addr1 << 1);
                if let Some(addr2) = addr2 {
                    self.oar2.enable_dual_addressing_mode();
                    self.oar2.set_address(addr2);
                }
            }
        }

        self.enable();

        Ok(())
    }

    #[inline]
    pub fn master_start(&mut self) -> Result<()> {
        while self.sr2.bus_is_busy() {}

        // Generate START condition
        self.cr1.generate_start_condition();
        while !(self.sr1.start_condition_is_generated() || self.sr1.is_bus_error_detected()) {}
        if self.sr1.is_bus_error_detected() {
            return Err(Error::BusError);
        }

        Ok(())
    }

    #[inline]
    pub fn master_write_address(&mut self, addr: u8, read: bool) -> Result<()> {
        // Send ADDRESS
        if read {
            self.dr.write_byte((addr << 1) | 1u8);
        } else {
            self.dr.write_byte((addr << 1) & !1u8);
        }
        while !(self.sr1.address_is_sent() || self.sr1.is_ack_failure_detected()) {}
        if self.sr1.is_bus_error_detected() {
            return Err(Error::NoSlaveAddress(addr));
        }
        self.sr1.get();
        self.sr2.get();

        Ok(())
    }

    #[inline]
    pub fn master_read_byte(&mut self) -> Result<u8> {
        while !self.sr1.rx_is_not_empty() {}
        Ok(self.dr.read_byte())
    }

    #[inline]
    pub fn master_write_byte(&mut self, byte: u8) -> Result<()> {
        while !self.sr1.tx_is_empty() {}
        Ok(self.dr.write_byte(byte))
    }

    #[inline]
    pub fn master_write_bytes(&mut self, data: &[u8]) -> Result<()> {
        // Send DATA
        for byte in data {
            self.master_write_byte(*byte)?;
        }

        Ok(())
    }

    #[inline]
    pub fn master_stop(&mut self) -> Result<()> {
        // Wait for transfer end
        while !self.sr1.tx_is_empty() {}
        while !self.sr1.data_transfer_is_finished() {}

        // Send STOP condition
        self.cr1.generate_stop_condition();

        Ok(())
    }

    pub fn master_write_data(&mut self, addr: u8, data: &[u8]) -> Result<()> {
        self.master_start()?;
        self.master_write_address(addr, false)?;
        self.master_write_bytes(data)?;
        self.master_stop()
    }

    pub fn master_write_data_begin(&mut self, addr: u8, data: &[u8]) -> Result<()> {
        unsafe {
            let state = &mut *state_mut(&self);
            match state.status {
                Status::Ready => Ok(()),
                Status::BusyRx => Err(Error::BusyError("RX in progress")),
                Status::BusyTx => Err(Error::BusyError("TX in progress")),
                Status::Error(e) => Err(e),
            }?;

            state.addr = addr;
            state.tx_buf = (data.as_ptr(), data.len());
            state.status = Status::BusyTx;
        }

        self.cr1.generate_start_condition();
        self.cr2.enable_buffer_interrupt();
        self.cr2.enable_event_interrupt();
        self.cr2.enable_error_interrupt();

        Ok(())
    }

    #[inline]
    pub fn master_read_bytes(&mut self, data: &mut [u8]) -> Result<()> {
        let len = data.len();

        // Enable auto ACKing if receiving more than 1 byte
        self.cr1.set_ack(len > 1);

        // Clear ADDR flag
        if self.sr1.address_is_sent() {
            self.sr1.get();
            self.sr2.get();
        }

        if len == 1 {
            self.cr1.generate_stop_condition();

            data[0] = self.master_read_byte()?;
        } else {
            for byte in &mut data[..len - 2] {
                *byte = self.master_read_byte()?;
            }

            self.cr1.set_ack(false);
            self.cr1.generate_stop_condition();

            for byte in &mut data[len - 2..] {
                *byte = self.master_read_byte()?;
            }
        }

        self.cr1.set_ack(true);

        Ok(())
    }

    pub fn master_read_data(&mut self, addr: u8, data: &mut [u8]) -> Result<()> {
        self.master_start()?;
        self.master_write_address(addr, true)?;
        self.master_read_bytes(data)
    }

    pub fn master_read_data_begin(&mut self, addr: u8, data: &mut [u8]) -> Result<()> {
        unsafe {
            let state = &mut *state_mut(&self);
            match state.status {
                Status::Ready => Ok(()),
                Status::BusyRx => Err(Error::BusyError("RX in progress")),
                Status::BusyTx => Err(Error::BusyError("TX in progress")),
                Status::Error(e) => Err(e),
            }?;

            state.addr = addr;
            state.rx_buf = (data.as_mut_ptr(), data.len());
            state.status = Status::BusyRx;
        }

        self.cr1.generate_start_condition();
        self.cr2.enable_buffer_interrupt();
        self.cr2.enable_event_interrupt();
        self.cr2.enable_error_interrupt();

        Ok(())
    }
}

impl PeripheralClock for I2C {
    fn reset(&self) {
        let ptr = self as *const Self;

        match ptr as usize {
            0x4000_5400 => {
                rcc().apb1rstr.i2c1_reset(true);
                rcc().apb1rstr.i2c1_reset(false);
            }
            0x4000_5800 => {
                rcc().apb1rstr.i2c2_reset(true);
                rcc().apb1rstr.i2c2_reset(false);
            }
            0x4000_5c00 => {
                rcc().apb1rstr.i2c3_reset(true);
                rcc().apb1rstr.i2c3_reset(false);
            }
            _ => panic!(),
        }
    }

    fn enable_clock(&self) {
        let ptr = self as *const Self;

        match ptr as usize {
            0x4000_5400 => rcc().apb1enr.i2c1_enable(),
            0x4000_5800 => rcc().apb1enr.i2c2_enable(),
            0x4000_5c00 => rcc().apb1enr.i2c3_enable(),
            _ => panic!(),
        }
    }

    fn disable_clock(&self) {
        let ptr = self as *const Self;

        match ptr as usize {
            0x4000_5400 => rcc().apb1enr.i2c1_disable(),
            0x4000_5800 => rcc().apb1enr.i2c2_disable(),
            0x4000_5c00 => rcc().apb1enr.i2c3_disable(),
            _ => panic!(),
        }
    }
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusMode {
    I2C = 0b0,
    SMBus = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SMBusType {
    Device = 0b0,
    Host = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedMode {
    StandardMode = 0b00,
    FastModeDuty2 = 0b10,
    FastModeDuty16_9 = 0b11,
}

#[derive(Debug)]
pub enum I2CMode {
    Master,
    Slave {
        addr1: u32,
        addr2: Option<u32>,
    },
}

pub fn i2c1() -> &'static mut I2C {
    peripheral(0x4000_5400)
}

pub fn i2c2() -> &'static mut I2C {
    peripheral(0x4000_5800)
}

pub fn i2c3() -> &'static mut I2C {
    peripheral(0x4000_5c00)
}
