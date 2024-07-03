use core::ptr::{ self, addr_of_mut };

use super::*;

pub(super) static mut USART1_STATE: State = State::new();
pub(super) static mut USART2_STATE: State = State::new();
pub(super) static mut USART3_STATE: State = State::new();
pub(super) static mut USART4_STATE: State = State::new();
pub(super) static mut USART5_STATE: State = State::new();
pub(super) static mut USART6_STATE: State = State::new();
pub(super) static mut USART7_STATE: State = State::new();
pub(super) static mut USART8_STATE: State = State::new();

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

pub(super) unsafe fn state_mut(usart: &USART) -> *mut State {
    let ptr = usart as *const USART;

    match ptr as usize {
        0x4001_1000 => addr_of_mut!(USART1_STATE),
        0x4000_4400 => addr_of_mut!(USART2_STATE),
        0x4000_4800 => addr_of_mut!(USART3_STATE),
        0x4000_4c00 => addr_of_mut!(USART4_STATE),
        0x4000_5000 => addr_of_mut!(USART5_STATE),
        0x4001_1400 => addr_of_mut!(USART6_STATE),
        0x4000_7800 => addr_of_mut!(USART7_STATE),
        0x4000_7c00 => addr_of_mut!(USART8_STATE),
        _ => panic!(),
    }
}

unsafe fn usart_irq_handler(usart: &mut USART, state: *mut State) {
    let state = &mut *state;

    if usart.cr1.is_tx_empty_interrupt_enabled() && usart.sr.tx_is_empty() {
        // handle TX
        match usart.cr1.get_word_length() {
            WordLength::EightBits => {
                usart.dr.write_data(*state.tx_buf.0 as u16);
                state.tx_buf = (state.tx_buf.0.add(1), state.tx_buf.1 - 1);
            }
            WordLength::NineBits => {
                if usart.cr1.is_parity_control_enabled() {
                    usart.dr.write_data(*state.tx_buf.0 as u16);
                    state.tx_buf = (state.tx_buf.0.add(1), state.tx_buf.1 - 1);
                } else {
                    usart.dr.write_data(*(state.tx_buf.0 as *const u16));
                    state.tx_buf = (state.tx_buf.0.add(2), state.tx_buf.1 - 2);
                }
            }
        }

        if state.tx_buf.1 == 0 {
            usart.cr1.disable_tx_empty_interrupt();

            state.tx_buf = (ptr::null(), 0);
            state.status = Status::Ready;
        }
    }

    if usart.cr1.is_rx_not_empty_interrupt_enabled() && usart.sr.rx_is_not_empty() {
        // handle RX
        match usart.cr1.get_word_length() {
            WordLength::EightBits => {
                *state.rx_buf.0 = usart.dr.read_data() as u8;
                state.rx_buf = (state.rx_buf.0.add(1), state.rx_buf.1 - 1);
            }
            WordLength::NineBits => {
                if usart.cr1.is_parity_control_enabled() {
                    *state.rx_buf.0 = usart.dr.read_data() as u8;
                    state.rx_buf = (state.rx_buf.0.add(1), state.rx_buf.1 - 1);
                } else {
                    *(state.rx_buf.0 as *mut u16) = usart.dr.read_data();
                    state.rx_buf = (state.rx_buf.0.add(2), state.rx_buf.1 - 2);
                }
            }
        }

        if state.rx_buf.1 == 0 {
            usart.cr1.disable_rx_not_empty_interrupt();

            state.rx_buf = (ptr::null_mut(), 0);
            state.status = Status::Ready;
        }
    }

    if usart.cr3.is_error_interrupt_enabled() {
        usart.cr3.disable_error_interrupt();

        match state.status {
            Status::BusyRx => {
                usart.cr1.disable_rx_not_empty_interrupt();
                usart.dr.read_data();
                usart.sr.get();
            }
            Status::BusyTx => {
                usart.cr1.disable_tx_empty_interrupt();
            }
            _ => (),
        }

        if usart.sr.is_overrun() {
            state.status = Status::Error(Error::OverrunError);
        } else if usart.sr.is_parity_error() {
            state.status = Status::Error(Error::ParityError);
        }
    }
}

/// USART1 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq37_handler() {
    usart_irq_handler(usart1(), addr_of_mut!(USART1_STATE));
}

/// USART2 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq38_handler() {
    usart_irq_handler(usart2(), addr_of_mut!(USART2_STATE));
}

/// USART3 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq39_handler() {
    usart_irq_handler(usart3(), addr_of_mut!(USART3_STATE));
}

/// UART4 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq52_handler() {
    usart_irq_handler(uart4(), addr_of_mut!(USART4_STATE));
}

/// UART5 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq53_handler() {
    usart_irq_handler(uart5(), addr_of_mut!(USART5_STATE));
}

/// USART6 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq71_handler() {
    usart_irq_handler(usart6(), addr_of_mut!(USART6_STATE));
}

/// UART7 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq82_handler() {
    usart_irq_handler(uart7(), addr_of_mut!(USART7_STATE));
}

/// UART8 interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq83_handler() {
    usart_irq_handler(uart8(), addr_of_mut!(USART8_STATE));
}
