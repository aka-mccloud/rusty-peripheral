use core::ptr::{ self, addr_of_mut };

use super::*;

pub(super) static mut I2C1_STATE: State = State::new();
pub(super) static mut I2C2_STATE: State = State::new();
pub(super) static mut I2C3_STATE: State = State::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Status {
    Ready,
    BusyRx,
    BusyTx,
    Error(Error),
}

pub(super) struct State {
    pub(super) addr: u8,
    pub(super) rx_buf: (*mut u8, usize),
    pub(super) tx_buf: (*const u8, usize),
    pub(super) status: Status,
}

impl State {
    const fn new() -> Self {
        Self {
            addr: 0,
            rx_buf: (ptr::null_mut(), 0),
            tx_buf: (ptr::null(), 0),
            status: Status::Ready,
        }
    }
}

pub(super) unsafe fn state_mut(i2c: &I2C) -> *mut State {
    let ptr = i2c as *const I2C;

    match ptr as usize {
        0x4000_5400 => addr_of_mut!(I2C1_STATE),
        0x4000_5800 => addr_of_mut!(I2C2_STATE),
        0x4000_5c00 => addr_of_mut!(I2C3_STATE),
        _ => panic!(),
    }
}

unsafe fn i2c_irq_event_handler(i2c: &mut I2C, state: *mut State) {
    let state = &mut *state;

    if !i2c.cr2.event_interrupt_is_enabled() {
        return;
    }
    if i2c.sr1.start_condition_is_generated() {
        match state.status {
            Status::Ready => {
                state.status = Status::Error(Error::InitError("logic error, not busy"));
            }
            Status::BusyRx => {
                i2c.dr.write_byte((state.addr << 1) | 1u8);
            }
            Status::BusyTx => {
                i2c.dr.write_byte((state.addr << 1) & !1u8);
            }
            _ => (),
        }
    }

    if i2c.sr1.address_is_sent() {
        if i2c.sr2.is_master() && state.status == Status::BusyRx && state.rx_buf.1 == 1 {
            i2c.cr1.set_ack(false);
        }

        i2c.sr1.get();
        i2c.sr2.get();
    }

    if i2c.sr1.data_transfer_is_finished() && state.tx_buf.1 == 0 {
        match state.status {
            Status::Ready => {
                state.status = Status::Error(Error::InitError("logic error, not busy"));
            }
            Status::BusyTx => {
                if i2c.sr1.tx_is_empty() {
                    i2c.cr1.generate_stop_condition();
                    state.tx_buf = (ptr::null_mut(), 0);
                    state.status = Status::Ready;
                }
            }
            _ => (),
        }
    }

    if i2c.sr1.stop_condition_is_detected() {
        i2c.cr1.enable_peripheral();
    }

    if !i2c.cr2.buffer_interrupt_is_enabled() {
        return;
    }

    if i2c.sr1.tx_is_empty() && i2c.sr2.is_master() {
        match state.status {
            Status::Ready => {
                state.status = Status::Error(Error::InitError("logic error, not busy"));
            }
            Status::BusyRx => {
                state.status = Status::Error(Error::InitError("logic error, busy in rx"));
            }
            Status::BusyTx if state.tx_buf.1 > 0 => {
                i2c.dr.write_byte(*state.tx_buf.0);
                state.tx_buf = (state.tx_buf.0.add(1), state.tx_buf.1 - 1);
            }
            _ => (),
        }
    }

    if i2c.sr1.rx_is_not_empty() && i2c.sr2.is_master() {
        match state.status {
            Status::Ready => {
                state.status = Status::Error(Error::InitError("logic error, not busy"));
            }
            Status::BusyRx if state.rx_buf.1 > 0 => {
                if state.rx_buf.1 == 2 {
                    i2c.cr1.set_ack(false);
                }

                *state.rx_buf.0 = i2c.dr.read_byte();
                state.rx_buf = (state.rx_buf.0.add(1), state.rx_buf.1 - 1);

                if state.rx_buf.1 == 0 {
                    i2c.cr1.generate_stop_condition();
        
                    state.rx_buf = (ptr::null_mut(), 0);
                    state.status = Status::Ready;
                }
            }
            Status::BusyTx => {
                state.status = Status::Error(Error::InitError("logic error, busy in tx"));
            }
            _ => (),
        }
    }
}

unsafe fn i2c_irq_error_handler(i2c: &mut I2C, state: *mut State) {
    let state = &mut *state;

    if i2c.sr1.is_bus_error_detected() {
        // handle bus error
        i2c.cr2.disable_error_interrupt();
        i2c.cr2.disable_event_interrupt();

        state.status = Status::Error(Error::BusError);
    }
}

/// I2C1_EV interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq31_handler() {
    i2c_irq_event_handler(i2c1(), addr_of_mut!(I2C1_STATE));
}

/// I2C1_ER interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq32_handler() {
    i2c_irq_error_handler(i2c1(), addr_of_mut!(I2C1_STATE));
}

/// I2C2_EV interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq33_handler() {
    i2c_irq_event_handler(i2c2(), addr_of_mut!(I2C2_STATE));
}

/// I2C2_ER interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq34_handler() {
    i2c_irq_error_handler(i2c2(), addr_of_mut!(I2C2_STATE));
}

/// I2C3_EV interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq72_handler() {
    i2c_irq_event_handler(i2c3(), addr_of_mut!(I2C3_STATE));
}

/// I2C3_ER interrupt handler
#[no_mangle]
unsafe extern "C" fn __irq73_handler() {
    i2c_irq_error_handler(i2c3(), addr_of_mut!(I2C3_STATE));
}
