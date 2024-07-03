use core::ptr::{ self, addr_of_mut };

use super::*;

pub(super) static mut SPI1_STATE: State = State::new();
pub(super) static mut SPI2_STATE: State = State::new();
pub(super) static mut SPI3_STATE: State = State::new();
pub(super) static mut SPI4_STATE: State = State::new();
pub(super) static mut SPI5_STATE: State = State::new();
pub(super) static mut SPI6_STATE: State = State::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Status {
    Ready,
    BusyRx,
    BusyTx,
    Error(Error)
}

pub(super) struct State {
    pub(super) rx_buf: (*mut u8, usize),
    pub(super) tx_buf: (*const u8, usize),
    pub(super) status: Status,
}

impl State {
    const fn new() -> Self {
        Self {
            rx_buf: (ptr::null_mut(), 0),
            tx_buf: (ptr::null(), 0),
            status: Status::Ready,
        }
    }
}

pub(super) unsafe fn state_mut(spi: &SPI) -> *mut State {
    let ptr = spi as *const SPI;

    match ptr as usize {
        0x4001_3000 => addr_of_mut!(SPI1_STATE),
        0x4000_3800 => addr_of_mut!(SPI2_STATE),
        0x4000_3c00 => addr_of_mut!(SPI3_STATE),
        0x4001_3400 => addr_of_mut!(SPI4_STATE),
        0x4001_5000 => addr_of_mut!(SPI5_STATE),
        0x4001_5400 => addr_of_mut!(SPI6_STATE),
        _ => panic!(),
    }
}

unsafe fn spi_irq_handler(spi: &mut SPI, state: *mut State) {
    let state = &mut *state;

    if spi.cr2.tx_empty_interrupt_is_enabled() && spi.sr.tx_is_empty() {
        // handle TX
        match spi.cr1.get_data_frame_format() {
            DataFrameFormat::Format8Bit => {
                spi.dr.write_data(*state.tx_buf.0 as u16);
                state.tx_buf = (state.tx_buf.0.add(1), state.tx_buf.1 - 1);
            }
            DataFrameFormat::Format16Bit => {
                spi.dr.write_data(*(state.tx_buf.0 as *const u16));
                state.tx_buf = (state.tx_buf.0.add(2), state.tx_buf.1 - 2);
            }
        }

        if state.tx_buf.1 == 0 {
            spi.cr2.disable_tx_empty_interrupt();

            state.tx_buf = (ptr::null(), 0);
            state.status = Status::Ready;
        }
    }

    if spi.cr2.rx_not_empty_interrupt_is_enabled() && spi.sr.rx_is_not_empty() {
        // handle RX
        match spi.cr1.get_data_frame_format() {
            DataFrameFormat::Format8Bit => {
                *state.rx_buf.0 = spi.dr.read_data() as u8;
                state.rx_buf = (state.rx_buf.0.add(1), state.rx_buf.1 - 1);
            }
            DataFrameFormat::Format16Bit => {
                *(state.rx_buf.0 as *mut u16) = spi.dr.read_data();
                state.rx_buf = (state.rx_buf.0.add(2), state.rx_buf.1 - 2);
            }
        }

        if state.rx_buf.1 == 0 {
            spi.cr2.disable_rx_not_empty_interrupt();

            state.rx_buf = (ptr::null_mut(), 0);
            state.status = Status::Ready;
        }
    }

    if spi.cr2.error_interrupt_is_enabled() && spi.sr.is_overrun() {
        // handle overrun error
        spi.cr2.disable_error_interrupt();

        match state.status {
            Status::BusyRx => {
                spi.cr2.disable_rx_not_empty_interrupt();
                spi.dr.read_data();
                spi.sr.get();
            }
            Status::BusyTx => {
                spi.cr2.disable_tx_empty_interrupt();
            }
            _ => (),
        }

        state.status = Status::Error(Error::OverrunError);
    }
}

/// SPI1 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq35_handler() {
    spi_irq_handler(spi1(), addr_of_mut!(SPI1_STATE));
}

/// SPI2 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq36_handler() {
    spi_irq_handler(spi2(), addr_of_mut!(SPI2_STATE));
}

/// SPI3 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq51_handler() {
    spi_irq_handler(spi3(), addr_of_mut!(SPI3_STATE));
}

/// SPI4 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq84_handler() {
    spi_irq_handler(spi4(), addr_of_mut!(SPI4_STATE));
}

/// SPI5 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq85_handler() {
    spi_irq_handler(spi5(), addr_of_mut!(SPI5_STATE));
}

/// SPI6 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq86_handler() {
    spi_irq_handler(spi6(), addr_of_mut!(SPI6_STATE));
}
