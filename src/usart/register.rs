use register::{ field::derive::RegisterField, register };

#[register(u32)]
pub(super) struct StatusRegister {
    #[bits(1, r, get = is_parity_error)]
    pub(super) PE: bool,

    #[bits(1, r, get = is_frame_error)]
    pub(super) FE: bool,

    #[bits(1, r, get = is_noise_detected)]
    pub(super) NF: bool,

    #[bits(1, r, get = is_overrun)]
    pub(super) ORE: bool,

    #[bits(1, r, get = is_idle_line)]
    pub(super) IDLE: bool,

    #[bits(1, rc, get = rx_is_not_empty, clear = rx_clear_not_empty)]
    pub(super) RXNE: bool,

    #[bits(1, rc, get = tx_is_complete, clear = tx_clear_complete)]
    pub(super) TC: bool,

    #[bits(1, r, get = tx_is_empty)]
    pub(super) TXE: bool,

    #[bits(1, rc, get = is_lin_break, clear = clear_lin_break)]
    pub(super) LBD: bool,

    #[bits(1, rc, get = cts_is_toggled, clear = clear_cts)]
    pub(super) CTS: bool,

    #[bits(22)]
    __: u32,
}

#[register(u32)]
pub(super) struct DataRegister {
    #[bits(9, rw, get = read_data, set = write_data)]
    pub(super) DR: u16,

    #[bits(23)]
    __: u32,
}

#[register(u32)]
pub(super) struct BaudRateRegister {
    #[bits(4, rw, get = get_fraction, set = set_fraction)]
    pub(super) fraction: u8,

    #[bits(12, rw, get = get_mantissa, set = set_mantissa)]
    pub(super) mantissa: u16,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub(super) struct ControlRegister1 {
    #[bits(1, rwc, get = is_send_break, set = send_break, clear = clear_break)]
    pub(super) SBK: bool,

    #[bits(1, rwc, get = is_mute_mode, set = set_mute_mode, clear = set_active_mode)]
    pub(super) RWU: bool,

    #[bits(1, rwc, get = is_rx_enabled, set = enable_rx, clear = disable_rx)]
    pub(super) RE: bool,

    #[bits(1, rwc, get = is_tx_enabled, set = enable_tx, clear = disable_tx)]
    pub(super) TE: bool,

    #[bits(
        1,
        rwc,
        get = is_idle_interrupt_enabled,
        set = enable_idle_interrupt,
        clear = disable_idle_interrupt
    )]
    pub(super) IDLEIE: bool,

    #[bits(
        1,
        rwc,
        get = is_rx_not_empty_interrupt_enabled,
        set = enable_rx_not_empty_interrupt,
        clear = disable_rx_not_empty_interrupt
    )]
    pub(super) RXNEIE: bool,

    #[bits(
        1,
        rwc,
        get = is_tx_complete_interrupt_enabled,
        set = enable_tx_complete_interrupt,
        clear = disable_tx_complete_interrupt
    )]
    pub(super) TCIE: bool,

    #[bits(
        1,
        rwc,
        get = is_tx_empty_interrupt_enabled,
        set = enable_tx_empty_interrupt,
        clear = disable_tx_empty_interrupt
    )]
    pub(super) TXEIE: bool,

    #[bits(
        1,
        rwc,
        get = is_parity_interrupt_enabled,
        set = enable_parity_interrupt,
        clear = disable_parity_interrupt
    )]
    pub(super) PEIE: bool,

    #[bits(1, rw, get = get_parity, set = set_parity)]
    pub(super) PS: Parity,

    #[bits(
        1,
        rwc,
        get = is_parity_control_enabled,
        set = enable_parity_control,
        clear = disable_parity_control
    )]
    pub(super) PCE: bool,

    #[bits(1, rw, get = get_wakeup_method, set = set_wakeup_method)]
    pub(super) WAKE: WakeupMethod,

    #[bits(1, rw, get = get_word_length, set = set_word_length)]
    pub(super) M: WordLength,

    #[bits(1, rwc, get = is_usart_enabled, set = enable_usart, clear = disable_usart)]
    pub(super) UE: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = get_oversampling_mode, set = set_oversampling_mode)]
    pub(super) OVER8: OversamplingMode,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub(super) struct ControlRegister2 {
    #[bits(4, rw, get = get_address, set = set_address)]
    pub(super) ADD: u8,

    #[bits(1)]
    __: u32,

    #[bits(
        1,
        rwc,
        get = is_lin_break_11bit_detection,
        set = set_lin_break_11bit_detection,
        clear = set_lin_break_10bit_detection
    )]
    pub(super) LBDL: bool,

    #[bits(
        1,
        rwc,
        get = is_lin_break_detection_interrupt_enabled,
        set = enable_lin_break_detection_interrupt,
        clear = disable_lin_break_detection_interrupt
    )]
    pub(super) LBDLIE: bool,

    #[bits(1)]
    __: u32,

    #[bits(
        1,
        rwc,
        get = is_last_bit_clock_pulse_enabled,
        set = enable_last_bit_clock_pulse,
        clear = disable_last_bit_clock_pulse
    )]
    pub(super) LBCL: bool,

    #[bits(1, rw, get = get_clock_phase, set = set_clock_phase)]
    pub(super) CPHA: ClockPhase,

    #[bits(1, rw, get = get_clock_polarity, set = set_clock_polarity)]
    pub(super) CPOL: ClockPolarity,

    #[bits(1, rwc, get = is_clock_pin_enabled, set = enable_clock_pin, clear = disable_clock_pin)]
    pub(super) CLKEN: bool,

    #[bits(2, rw, get = get_stop_bits, set = set_stop_bits)]
    pub(super) STOP: StopBits,

    #[bits(1, rwc, get = is_lin_mode_enabled, set = enable_lin_mode, clear = disable_lin_mode)]
    pub(super) LINEN: bool,

    #[bits(17)]
    __: u32,
}

#[register(u32)]
pub(super) struct ControlRegister3 {
    #[bits(
        1,
        rwc,
        get = is_error_interrupt_enabled,
        set = enable_error_interrupt,
        clear = disable_error_interrupt
    )]
    pub(super) EIE: bool,

    #[bits(1, rwc, get = is_irda_mode_enabled, set = enable_irda_mode, clear = disable_irda_mode)]
    pub(super) IREN: bool,

    #[bits(
        1,
        rwc,
        get = is_irda_low_power_mode_enabled,
        set = enable_irda_low_power_mode,
        clear = disable_irda_low_power_mode
    )]
    pub(super) IRLP: bool,

    #[bits(
        1,
        rwc,
        get = is_half_duplex_mode_enabled,
        set = enable_half_duplex_mode,
        clear = disable_half_duplex_mode
    )]
    pub(super) HDSEL: bool,

    #[bits(
        1,
        rwc,
        get = is_smartcard_nack_enabled,
        set = enable_smartcard_nack,
        clear = disable_smartcard_nack
    )]
    pub(super) NACK: bool,

    #[bits(
        1,
        rwc,
        get = is_smartcard_mode_enabled,
        set = enable_smartcard_mode,
        clear = disable_smartcard_mode
    )]
    pub(super) SCEN: bool,

    #[bits(1, rwc, get = is_rx_dma_enabled, set = enable_rx_dma, clear = disable_rx_dma)]
    pub(super) DMAR: bool,

    #[bits(1, rwc, get = is_tx_dma_enabled, set = enable_tx_dma, clear = disable_tx_dma)]
    pub(super) DMAT: bool,

    #[bits(1, rwc, get = is_rts_enabled, set = enable_rts, clear = disable_rts)]
    pub(super) RTSE: bool,

    #[bits(1, rwc, get = is_cts_enabled, set = enable_cts, clear = disable_cts)]
    pub(super) CTSE: bool,

    #[bits(
        1,
        rwc,
        get = is_cts_interrupt_enabled,
        set = enable_cts_interrupt,
        clear = disable_cts_interrupt
    )]
    pub(super) CTSIE: bool,

    #[bits(1, rw, get = get_sample_method, set = set_sample_method)]
    pub(super) ONEBIT: SampleMethod,

    #[bits(20)]
    __: u32,
}

#[register(u32)]
pub(super) struct GuardTimeAndPrescalerRegister {
    #[bits(8, rw, get = get_prescaler, set = set_prescaler)]
    pub(super) PSC: u8,

    #[bits(8, rw, get = get_guard_time, set = set_guard_time)]
    pub(super) GT: u8,

    #[bits(16)]
    __: u32,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Even = 0b0,
    Odd = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeupMethod {
    IdleLine = 0b0,
    AddressMark = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordLength {
    EightBits = 0b0,
    NineBits = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OversamplingMode {
    By16 = 0b0,
    By8 = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPhase {
    FirstClockTransition = 0b0,
    SecondClockTransition = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPolarity {
    IdleLow = 0b0,
    IdleHigh = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One = 0b00,
    ZeroAndHalf = 0b01,
    Two = 0b10,
    OneAndHalf = 0b11,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleMethod {
    ThreeBit = 0b0,
    OneBit = 0b1,
}
