#![allow(dead_code)]

use core::fmt::{ self };

use crate::{
    gpio::{ self, pin::Pin, port::Port, OutputType, PinConfig, Speed, Pull },
    rcc,
};

use self::register::*;

mod register;

pub use self::register::{ BusMode, SMBusType, SpeedMode };

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

pub fn get_i2c1() -> &'static mut I2C {
    let addr = 0x4000_5400u32;

    unsafe {
        let ptr: *mut I2C = addr as *mut I2C;
        &mut *ptr
    }
}

pub fn get_i2c2() -> &'static mut I2C {
    let addr = 0x4000_5800u32;

    unsafe {
        let ptr: *mut I2C = addr as *mut I2C;
        &mut *ptr
    }
}

pub fn get_i2c3() -> &'static mut I2C {
    let addr = 0x4000_5c00u32;

    unsafe {
        let ptr: *mut I2C = addr as *mut I2C;
        &mut *ptr
    }
}

#[derive(Debug)]
pub enum Error {
    InitError(&'static str),
    BusError(&'static str),
    NoSlaveAddress(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitError(e) => f.write_fmt(format_args!("InitError: {}", e)),
            Error::BusError(e) => f.write_fmt(format_args!("BusError: {}", e)),
            Error::NoSlaveAddress(a) => f.write_fmt(format_args!("No Slave with address: {}", a)),
        }
    }
}

impl I2C {
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
        mode: I2CMode,
        speed_mode: SpeedMode,
        scl_freq: u32,
        scl_pin: (Port, Pin),
        sda_pin: (Port, Pin)
    ) -> Result<(), Error> {
        let rcc = rcc::get_rcc();
        let ptr = self as *const I2C;

        match ptr as u32 {
            0x4000_5400u32 => {
                rcc.apb1enr.i2c1_enable(true);
            }
            0x4000_5800u32 => {
                rcc.apb1enr.i2c2_enable(true);
            }
            0x4000_5c00u32 => {
                rcc.apb1enr.i2c3_enable(true);
            }
            _ => panic!(),
        }

        let mut gpio_scl = gpio::get_port(scl_pin.0);
        let mut gpio_sda = gpio::get_port(sda_pin.0);

        gpio_scl.enable_clock();
        gpio_sda.enable_clock();

        gpio_scl.init_pins(
            scl_pin.1,
            PinConfig::Alternate(4, OutputType::OpenDrain, Speed::VeryHigh, Pull::None)
        );
        gpio_sda.init_pins(
            sda_pin.1,
            PinConfig::Alternate(4, OutputType::OpenDrain, Speed::VeryHigh, Pull::None)
        );

        self.disable();

        if scl_freq > 100_000 && speed_mode == SpeedMode::StandardMode {
            return Err(Error::InitError("Frequency is too high"));
        }

        if scl_freq <= 100_000 && speed_mode != SpeedMode::StandardMode {
            return Err(Error::InitError("Frequency is too low"));
        }

        // calculate CCR
        let f_pclk1 = rcc.get_pclk1_freq();

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
                    self.oar2.enable_dual_addressing_mode(true);
                    self.oar2.set_address(addr2);
                }
            }
        }

        self.enable();

        Ok(())
    }

    pub fn reset(&self) {
        let ptr: *const I2C = self as *const I2C;
        let addr = ptr as u32;

        match addr {
            0x4000_5400u32 => {
                rcc::get_rcc().apb1rstr.i2c1_reset(true);
                rcc::get_rcc().apb1rstr.i2c1_reset(false);
            }
            0x4000_5800u32 => {
                rcc::get_rcc().apb1rstr.i2c2_reset(true);
                rcc::get_rcc().apb1rstr.i2c2_reset(false);
            }
            0x4000_5c00u32 => {
                rcc::get_rcc().apb1rstr.i2c3_reset(true);
                rcc::get_rcc().apb1rstr.i2c3_reset(false);
            }
            _ => panic!(),
        }
    }

    #[inline]
    pub fn master_start(&mut self) -> Result<(), Error> {
        while self.sr2.bus_is_busy() {}

        // Generate START condition
        self.cr1.generate_start_condition(true);
        while !(self.sr1.start_condition_is_generated() || self.sr1.is_bus_error_detected()) {}
        if self.sr1.is_bus_error_detected() {
            return Err(Error::BusError("Misplaced Start condition"));
        }

        Ok(())
    }

    #[inline]
    pub fn master_write_address(&mut self, addr: u8, read: bool) -> Result<(), Error> {
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
    pub fn master_read_byte(&mut self) -> Result<u8, Error> {
        while !self.sr1.rx_not_empty() {}
        Ok(self.dr.read_byte())
    }

    #[inline]
    pub fn master_write_byte(&mut self, byte: u8) -> Result<(), Error> {
        while !self.sr1.tx_is_empty() {}
        Ok(self.dr.write_byte(byte))
    }

    #[inline]
    pub fn master_write_bytes(&mut self, data: &[u8]) -> Result<(), Error> {
        // Send DATA
        for byte in data {
            self.master_write_byte(*byte)?;
        }

        Ok(())
    }

    #[inline]
    pub fn master_stop(&mut self) -> Result<(), Error> {
        // Wait for transfer end
        while !self.sr1.tx_is_empty() {}
        while !self.sr1.data_transfer_is_finished() {}

        // Send STOP condition
        self.cr1.generate_stop_condition(true);

        Ok(())
    }

    pub fn master_write_data(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        self.master_start()?;
        self.master_write_address(addr, false)?;
        self.master_write_bytes(data)?;
        self.master_stop()?;

        Ok(())
    }

    #[inline]
    pub fn master_read_bytes(&mut self, data: &mut [u8]) -> Result<(), Error> {
        let len = data.len();

        // Enable auto ACKing if receiving more than 1 byte
        self.cr1.set_ack(len > 1);

        // Clear ADDR flag
        if self.sr1.address_is_sent() {
            self.sr1.get();
            self.sr2.get();
        }

        if len == 1 {
            self.cr1.generate_stop_condition(true);

            data[0] = self.master_read_byte()?;
        } else {
            for byte in &mut data[..len - 2] {
                *byte = self.master_read_byte()?;
            }

            self.cr1.set_ack(false);
            self.cr1.generate_stop_condition(true);

            for byte in &mut data[len - 2..] {
                *byte = self.master_read_byte()?;
            }
        }

        self.cr1.set_ack(true);

        Ok(())
    }

    pub fn master_read_data(&mut self, addr: u8, data: &mut [u8]) -> Result<(), Error> {
        self.master_start()?;
        self.master_write_address(addr, true)?;

        self.master_read_bytes(data)
    }
}

pub enum I2CMode {
    Master,
    Slave {
        addr1: u32,
        addr2: Option<u32>,
    },
}
