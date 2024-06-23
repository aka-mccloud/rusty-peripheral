use core::ptr;

use super::{ spi1, spi2, spi3, spi4, spi5, spi6, DataFrameFormat, SPI };

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

pub(super) unsafe fn state_mut(spi: &SPI) -> &mut State {
    let ptr = spi as *const SPI;

    match ptr as u32 {
        0x4001_3000u32 => { &mut SPI1_STATE }
        0x4000_3800u32 => { &mut SPI2_STATE }
        0x4000_3c00u32 => { &mut SPI3_STATE }
        0x4001_3400u32 => { &mut SPI4_STATE }
        0x4001_5000u32 => { &mut SPI5_STATE }
        0x4001_5400u32 => { &mut SPI6_STATE }
        _ => panic!(),
    }
}

fn spi_irq_handler(spi: &mut SPI, state: &mut State) {
    if spi.cr2.tx_empty_interrupt_is_enabled() && spi.sr.tx_is_empty() {
        // handle TX
        unsafe {
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
        }
        if state.tx_buf.1 == 0 {
            spi.cr2.disable_tx_empty_interrupt();

            state.tx_buf = (ptr::null(), 0);
            state.status = Status::Ready;
        }
    }

    if spi.cr2.rx_not_empty_interrupt_is_enabled() && spi.sr.rx_is_not_empty() {
        // handle RX
        unsafe {
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
        }
        if state.rx_buf.1 == 0 {
            spi.cr2.disable_rx_not_empty_interrupt();

            state.rx_buf = (ptr::null_mut(), 0);
            state.status = Status::Ready;
        }
    }

    if spi.cr2.error_interrupt_is_enabled() && spi.sr.is_overrun() {
        // handle overrun error
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

        *state = State::new();
    }
}

/// SPI1 interrupt handler
#[no_mangle]
extern "C" fn __irq35_handler() {
    spi_irq_handler(spi1(), unsafe { &mut SPI1_STATE });
}

/// SPI2 interrupt handler
#[no_mangle]
extern "C" fn __irq36_handler() {
    spi_irq_handler(spi2(), unsafe { &mut SPI2_STATE });
}

/// SPI3 interrupt handler
#[no_mangle]
extern "C" fn __irq51_handler() {
    spi_irq_handler(spi3(), unsafe { &mut SPI3_STATE });
}

/// SPI4 interrupt handler
#[no_mangle]
extern "C" fn __irq84_handler() {
    spi_irq_handler(spi4(), unsafe { &mut SPI4_STATE });
}

/// SPI5 interrupt handler
#[no_mangle]
extern "C" fn __irq85_handler() {
    spi_irq_handler(spi5(), unsafe { &mut SPI5_STATE });
}

/// SPI6 interrupt handler
#[no_mangle]
extern "C" fn __irq86_handler() {
    spi_irq_handler(spi6(), unsafe { &mut SPI6_STATE });
}
