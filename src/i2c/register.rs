use register::register;

use super::{ BusMode, SMBusType, SpeedMode };

#[register(u32)]
pub(super) struct ControlRegister1 {
    #[bits(
        1,
        rwc,
        get = peripheral_is_enabled,
        set = enable_peripheral,
        clear = disable_peripheral
    )]
    pub(super) PE: bool,

    #[bits(1, rw, get = get_bus_mode, set = set_bus_mode)]
    pub(super) SMBUS: BusMode,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = get_smbus_type, set = set_smbus_type)]
    pub(super) SMBTYPE: SMBusType,

    #[bits(1, rwc, get = arp_is_enabled, set = enable_arp, clear = disable_arp)]
    pub(super) ENARP: bool,

    #[bits(1, rwc, get = pec_is_enabled, set = enable_pec, clear = disable_pec)]
    pub(super) ENPEC: bool,

    #[bits(
        1,
        rwc,
        get = general_call_is_enabled,
        set = enable_general_call,
        clear = disable_general_call
    )]
    pub(super) ENGC: bool,

    #[bits(
        1,
        rwc,
        get = clock_stretching_is_disabled,
        set = disable_clock_stretching,
        clear = enable_clock_stretching
    )]
    pub(super) NOSTRETCH: bool,

    #[bits(
        1,
        rwc,
        get = start_is_set,
        set = generate_start_condition,
        clear = clear_start_condition
    )]
    pub(super) START: bool,

    #[bits(1, rwc, get = stop_is_set, set = generate_stop_condition, clear = clear_stop_condition)]
    pub(super) STOP: bool,

    #[bits(1, rw, get = ack_is_set, set = set_ack)]
    pub(super) ACK: bool,

    #[bits(1, rw, get = pos_is_set, set = pos_set)]
    pub(super) POS: bool,

    #[bits(1, rw, get = pec_is_set, set = pec_set)]
    pub(super) PEC: bool,

    #[bits(1, rw, get = alert_is_set, set = set_alert)]
    pub(super) ALERT: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = is_under_reset_state, set = reset)]
    pub(super) SWRST: bool,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub struct ControlRegister2 {
    #[bits(6, rw, get = get_peripheral_clock_frequency, set = set_peripheral_clock_frequency)]
    pub(super) FREQ: u32,

    #[bits(2)]
    __: u32,

    #[bits(
        1,
        rwc,
        get = error_interrupt_is_enabled,
        set = enable_error_interrupt,
        clear = disable_error_interrupt
    )]
    pub(super) ITERREN: bool,

    #[bits(
        1,
        rwc,
        get = event_interrupt_is_enabled,
        set = enable_event_interrupt,
        clear = disable_event_interrupt
    )]
    pub(super) ITEVTEN: bool,

    #[bits(
        1,
        rwc,
        get = buffer_interrupt_is_enabled,
        set = enable_buffer_interrupt,
        clear = disable_buffer_interrupt
    )]
    pub(super) ITBUFEN: bool,

    #[bits(
        1,
        rwc,
        get = dma_requests_is_enabled,
        set = enable_dma_requests,
        clear = disable_dma_requests
    )]
    pub(super) DMAEN: bool,

    #[bits(1, rw, get = next_dma_eot_is_last, set = set_next_dma_eot_last)]
    pub(super) LAST: bool,

    #[bits(19)]
    __: u32,
}

#[register(u32)]
pub struct OwnAddressRegister1 {
    #[bits(10, rw, get = get_address, set = set_address)]
    pub(super) ADDR: u32,

    #[bits(4)]
    __: u32,

    #[bits(1, w, set = set_as_one)]
    pub(super) KEEP1: bool,

    #[bits(1, rw, get = is_10bit_address_mode, set = set_10bit_address_mode)]
    pub(super) ADDMODE: bool,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub struct OwnAddressRegister2 {
    #[bits(
        1,
        rwc,
        get = dual_addressing_mode_is_enabled,
        set = enable_dual_addressing_mode,
        clear = disable_dual_addressing_mode
    )]
    pub(super) ENDUAL: bool,

    #[bits(7, rw, get = get_address, set = set_address)]
    pub(super) ADDR: u32,

    #[bits(24)]
    __: u32,
}

#[register(u32)]
pub struct DataRegister {
    #[bits(8, rw, get = read_byte, set = write_byte)]
    pub(super) DR: u8,

    #[bits(24)]
    __: u32,
}

#[register(u32)]
pub struct StatusRegister1 {
    #[bits(1, r, get = start_condition_is_generated)]
    pub(super) SB: bool,

    #[bits(1, r, get = address_is_sent)]
    pub(super) ADDR: bool,

    #[bits(1, r, get = data_transfer_is_finished)]
    pub(super) BTF: bool,

    #[bits(1, r, get = address_header_is_sent)]
    pub(super) ADD10: bool,

    #[bits(1, r, get = stop_condition_is_detected)]
    pub(super) STOPF: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, r, get = rx_is_not_empty)]
    pub(super) RxNE: bool,

    #[bits(1, r, get = tx_is_empty)]
    pub(super) TxE: bool,

    #[bits(1, rc, get = is_bus_error_detected, clear = clear_bus_error)]
    pub(super) BERR: bool,

    #[bits(1, rc, get = is_arbitration_lost_detected, clear = clear_arbitration_lost)]
    pub(super) ARLO: bool,

    #[bits(1, rc, get = is_ack_failure_detected, clear = clear_ack_failure)]
    pub(super) AF: bool,

    #[bits(1, rc, get = is_overrun_underrun_detected, clear = clear_overrun_underrun)]
    pub(super) OVR: bool,

    #[bits(1, rc, get = is_pec_error_detected, clear = clear_pec_error)]
    pub(super) PECERR: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rc, get = is_timeout_detected, clear = clear_timeout)]
    pub(super) TIMEOUT: bool,

    #[bits(1, rc, get = is_smbus_alert_detected, clear = clear_smbus_alert)]
    pub(super) SMBALERT: bool,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub struct StatusRegister2 {
    #[bits(1, r, get = is_master)]
    pub(super) MSL: bool,

    #[bits(1, r, get = bus_is_busy)]
    pub(super) BUSY: bool,

    #[bits(1, r, get = is_transmitter)]
    pub(super) TRA: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, r, get = is_general_call_received)]
    pub(super) GENCALL: bool,

    #[bits(1, r, get = is_smbus_default_address_received)]
    pub(super) SMBDEFAULT: bool,

    #[bits(1, r, get = is_smbus_host_header_received)]
    pub(super) SMBHOST: bool,

    #[bits(1, r, get = is_secondary_address_matched)]
    pub(super) DUALF: bool,

    #[bits(8, r, get = get_pec_register)]
    pub(super) PEC: u8,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub struct ClockControlRegister {
    #[bits(11, rw, get = get_ccr, set = set_ccr)]
    pub(super) CCR: u32,

    #[bits(2)]
    __: u32,

    #[bits(2, rw, get = get_speed_mode, set = set_speed_mode)]
    pub(super) FSDUTY: SpeedMode,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub struct RiseTimeRegister {
    #[bits(6, rw, get = get_rise_time, set = set_rise_time)]
    pub(super) TRISE: u32,

    #[bits(26)]
    __: u32,
}

#[register(u32)]
pub struct FilterRegister {
    #[bits(4, rw, get = get_digital_noise_filter_value, set = set_digital_noise_filter_value)]
    pub(super) DNF: u8,

    #[bits(
        1,
        rwc,
        get = analog_noise_filter_is_disabled,
        set = disable_analog_noise_filter,
        clear = enable_analog_noise_filter
    )]
    pub(super) ANOFF: bool,

    #[bits(27)]
    __: u32,
}
