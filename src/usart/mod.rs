use core::{ fmt, mem::size_of, ptr, slice };

use irq::{ state_mut, Status };

use crate::{ peripheral, rcc::rcc, PeripheralClock };

use self::register::*;

pub use self::register::{ Parity, StopBits, WordLength };

mod register;
mod irq;

#[allow(unused)]
pub struct USART {
    /// Status Register
    sr: StatusRegister,

    /// Data Register
    dr: DataRegister,

    /// Baud Rate Register
    brr: BaudRateRegister,

    /// Control Register 1
    cr1: ControlRegister1,

    /// Control Register 2
    cr2: ControlRegister2,

    /// Control Register 3
    cr3: ControlRegister3,

    /// Guard Time and Prescaler Register
    gptr: GuardTimeAndPrescalerRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InitError(&'static str),
    OverrunError,
    ParityError,
    BusyError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitError(e) => f.write_fmt(format_args!("InitError: {}", e)),
            Error::OverrunError => f.write_str("Overrun Error"),
            Error::ParityError => f.write_str("Parity Error"),
            Error::BusyError(e) => f.write_fmt(format_args!("BusyError: {}", e)),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl USART {
    pub fn enable(&mut self) {
        self.cr1.enable_usart()
    }

    pub fn disable(&mut self) {
        self.cr1.disable_usart()
    }

    pub fn is_enabled(&self) -> bool {
        self.cr1.is_usart_enabled()
    }

    pub fn init(
        &mut self,
        mode: USARTMode,
        baud_rate: u32,
        word_length: WordLength,
        stop_bits: StopBits,
        oversampling: bool,
        parity: Option<Parity>,
        flow_control: Option<FlowControl>
    ) -> Result<()> {
        self.disable();

        match mode {
            USARTMode::TX => {
                self.cr1.enable_tx();
                self.cr1.disable_rx();
            }
            USARTMode::RX => {
                self.cr1.disable_tx();
                self.cr1.enable_rx();
            }
            USARTMode::TX_RX => {
                self.cr1.enable_tx();
                self.cr1.enable_rx();
            }
        }

        self.cr1.set_word_length(word_length);
        if let Some(parity) = parity {
            self.cr1.enable_parity_control();
            self.cr1.set_parity(parity);
        } else {
            self.cr1.disable_parity_control();
        }

        self.cr2.set_stop_bits(stop_bits);

        if let Some(flow_control) = flow_control {
            match flow_control {
                FlowControl::CTS => {
                    self.cr3.enable_cts();
                    self.cr3.disable_rts();
                }
                FlowControl::RTS => {
                    self.cr3.disable_cts();
                    self.cr3.enable_rts();
                }
                FlowControl::CTS_RTS => {
                    self.cr3.enable_cts();
                    self.cr3.enable_rts();
                }
            }
        } else {
            self.cr3.disable_cts();
            self.cr3.disable_rts();
        }

        let div = if oversampling {
            (25 * self.pclk_freq()) / (2 * baud_rate)
        } else {
            (25 * self.pclk_freq()) / (4 * baud_rate)
        };

        let mantissa = div / 100;
        let fraction = ((div % 100) * (if oversampling { 8 } else { 16 }) + 50) / 100;

        self.brr.set_mantissa(mantissa as u16);
        self.brr.set_fraction(fraction as u8);

        self.enable();

        Ok(())
    }

    #[inline]
    pub fn write_word(&mut self, word: u16) -> Result<()> {
        while !self.sr.tx_is_empty() {}
        Ok(self.dr.write_data(word))
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<()> {
        match self.cr1.get_word_length() {
            WordLength::EightBits => {
                for byte in data {
                    self.write_word(*byte as _)?;
                }
            }
            WordLength::NineBits => {
                if self.cr1.is_parity_control_enabled() {
                    for byte in data {
                        self.write_word(*byte as _)?;
                    }
                } else {
                    let data = unsafe {
                        slice::from_raw_parts(
                            data.as_ptr() as *const _,
                            data.len() / size_of::<u16>()
                        )
                    };
                    for word in data {
                        self.write_word(*word)?;
                    }
                }
            }
        }

        while !self.sr.tx_is_complete() {}

        Ok(())
    }

    pub fn write_data_begin(&mut self, data: &[u8]) -> Result<()> {
        unsafe {
            let state = &mut *state_mut(&self);
            (match state.status {
                Status::Ready => Ok(()),
                Status::BusyRx => Err(Error::BusyError("RX in progress")),
                Status::BusyTx => Err(Error::BusyError("TX in progress")),
                Status::Error(e) => Err(e),
            })?;

            state.tx_buf = (data.as_ptr(), data.len());
            state.status = Status::BusyTx;
        }

        self.cr1.enable_tx_empty_interrupt();

        Ok(())
    }

    #[inline]
    pub fn read_word(&mut self) -> Result<u16> {
        while !self.sr.rx_is_not_empty() {}
        Ok(self.dr.read_data())
    }

    pub fn read_data(&mut self, data: &mut [u8]) -> Result<()> {
        match self.cr1.get_word_length() {
            WordLength::EightBits => {
                for byte in data {
                    *byte = self.read_word()? as u8;
                }
            }
            WordLength::NineBits => {
                if self.cr1.is_parity_control_enabled() {
                    for byte in data {
                        *byte = self.read_word()? as u8;
                    }
                } else {
                    let data = unsafe {
                        slice::from_raw_parts_mut(
                            data.as_ptr() as *mut u16,
                            data.len() / size_of::<u16>()
                        )
                    };

                    for word in data {
                        *word = self.read_word()?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn read_data_begin(&mut self, data: &mut [u8]) -> Result<()> {
        unsafe {
            let state = &mut *state_mut(&self);
            (match state.status {
                Status::Ready => Ok(()),
                Status::BusyRx => Err(Error::BusyError("RX in progress")),
                Status::BusyTx => Err(Error::BusyError("TX in progress")),
                Status::Error(e) => Err(e),
            })?;

            state.rx_buf = (data.as_mut_ptr(), data.len());
            state.status = Status::BusyRx;
        }

        self.cr1.enable_rx_not_empty_interrupt();

        Ok(())
    }

    fn pclk_freq(&self) -> u32 {
        let ptr = ptr::from_ref(self);

        match ptr as usize {
            0x4001_1000 => rcc().pclk2_freq(),
            0x4000_4400 => rcc().pclk1_freq(),
            0x4000_4800 => rcc().pclk1_freq(),
            0x4000_4c00 => rcc().pclk1_freq(),
            0x4000_5000 => rcc().pclk1_freq(),
            0x4001_1400 => rcc().pclk2_freq(),
            0x4000_7800 => rcc().pclk1_freq(),
            0x4000_7c00 => rcc().pclk1_freq(),
            _ => panic!(),
        }
    }
}

impl PeripheralClock for USART {
    fn reset(&self) {
        let ptr = ptr::from_ref(self);

        match ptr as usize {
            0x4001_1000 => {
                rcc().apb2rstr.usart1_reset(true);
                rcc().apb2rstr.usart1_reset(false);
            }
            0x4000_4400 => {
                rcc().apb1rstr.usart2_reset(true);
                rcc().apb1rstr.usart2_reset(false);
            }
            0x4000_4800 => {
                rcc().apb1rstr.usart3_reset(true);
                rcc().apb1rstr.usart3_reset(false);
            }
            0x4000_4c00 => {
                rcc().apb1rstr.uart4_reset(true);
                rcc().apb1rstr.uart4_reset(false);
            }
            0x4000_5000 => {
                rcc().apb1rstr.uart5_reset(true);
                rcc().apb1rstr.uart5_reset(false);
            }
            0x4001_1400 => {
                rcc().apb2rstr.usart6_reset(true);
                rcc().apb2rstr.usart6_reset(false);
            }
            0x4000_7800 => {
                rcc().apb1rstr.uart7_reset(true);
                rcc().apb1rstr.uart7_reset(false);
            }
            0x4000_7c00 => {
                rcc().apb1rstr.uart8_reset(true);
                rcc().apb1rstr.uart8_reset(false);
            }
            _ => panic!(),
        }
    }

    fn enable_clock(&self) {
        let ptr = ptr::from_ref(self);

        match ptr as usize {
            0x4001_1000 => rcc().apb2enr.usart1_enable(),
            0x4000_4400 => rcc().apb1enr.usart2_enable(),
            0x4000_4800 => rcc().apb1enr.usart3_enable(),
            0x4000_4c00 => rcc().apb1enr.uart4_enable(),
            0x4000_5000 => rcc().apb1enr.uart5_enable(),
            0x4001_1400 => rcc().apb2enr.usart6_enable(),
            0x4000_7800 => rcc().apb1enr.uart7_enable(),
            0x4000_7c00 => rcc().apb1enr.uart8_enable(),
            _ => panic!(),
        }
    }

    fn disable_clock(&self) {
        let ptr = ptr::from_ref(self);

        match ptr as usize {
            0x4001_1000 => rcc().apb2enr.usart1_disable(),
            0x4000_4400 => rcc().apb1enr.usart2_disable(),
            0x4000_4800 => rcc().apb1enr.usart3_disable(),
            0x4000_4c00 => rcc().apb1enr.uart4_disable(),
            0x4000_5000 => rcc().apb1enr.uart5_disable(),
            0x4001_1400 => rcc().apb2enr.usart6_disable(),
            0x4000_7800 => rcc().apb1enr.uart7_disable(),
            0x4000_7c00 => rcc().apb1enr.uart8_disable(),
            _ => panic!(),
        }
    }
}

impl fmt::Write for USART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_data(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum USARTMode {
    TX,
    RX,
    TX_RX,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowControl {
    CTS,
    RTS,
    CTS_RTS,
}

pub fn usart1() -> &'static mut USART {
    peripheral(0x4001_1000)
}

pub fn usart2() -> &'static mut USART {
    peripheral(0x4000_4400)
}

pub fn usart3() -> &'static mut USART {
    peripheral(0x4000_4800)
}

pub fn uart4() -> &'static mut USART {
    peripheral(0x4000_4c00)
}

pub fn uart5() -> &'static mut USART {
    peripheral(0x4000_5000)
}

pub fn usart6() -> &'static mut USART {
    peripheral(0x4001_1400)
}

pub fn uart7() -> &'static mut USART {
    peripheral(0x4000_7800)
}

pub fn uart8() -> &'static mut USART {
    peripheral(0x4000_7c00)
}
