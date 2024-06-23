use register::{ field::derive::RegisterField, register };

#[register(u32)]
pub(super) struct ControlRegister1 {
    #[bits(1, rw, get = get_clock_phase, set = set_clock_phase)]
    pub(super) CPHA: ClockPhase,

    #[bits(1, rw, get = get_clock_polarity, set = set_clock_polarity)]
    pub(super) CPOL: ClockPolarity,

    #[bits(1, rw, get = get_mode, set = set_mode)]
    pub(super) MSTR: Mode,

    #[bits(3, rw, get = get_baud_rate, set = set_baud_rate)]
    pub(super) BR: BaudRate,

    #[bits(
        1,
        rwc,
        get = peripheral_is_enabled,
        set = enable_peripheral,
        clear = disable_peripheral
    )]
    pub(super) SPE: bool,

    #[bits(1, rw, get = get_frame_format, set = set_frame_format)]
    pub(super) SPE: FrameFormat,

    #[bits(1, rw, get = get_internal_slave_select, set = set_internal_slave_select)]
    pub(super) SSI: bool,

    #[bits(
        1,
        rwc,
        get = software_slave_management_is_enabled,
        set = enable_software_slave_management,
        clear = disable_software_slave_management
    )]
    pub(super) SSM: bool,

    #[bits(
        1,
        rwc,
        get = receive_only_is_enabled,
        set = enable_receive_only,
        clear = disable_receive_only
    )]
    pub(super) RXONLY: bool,

    #[bits(1, rw, get = get_data_frame_format, set = set_data_frame_format)]
    pub(super) DFF: DataFrameFormat,

    #[bits(
        1,
        rwc,
        get = crc_transfer_is_next,
        set = set_next_transfer_crc,
        clear = clear_next_transfer_crc
    )]
    pub(super) CRCNEXT: bool,

    #[bits(
        1,
        rwc,
        get = hardware_crc_is_enabled,
        set = enable_hardware_crc,
        clear = disable_hardware_crc
    )]
    pub(super) CRCEN: bool,

    #[bits(1, rw, get = get_bidirectional_mode, set = set_bidirectional_mode)]
    pub(super) BIDIOE: BidirectionalMode,

    #[bits(
        1,
        rwc,
        get = bidirectinal_mode_is_enabled,
        set = enable_bidirectional_mode,
        clear = disable_bidirectional_mode
    )]
    pub(super) BIDIMODE: bool,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub(super) struct ControlRegister2 {
    #[bits(1, rwc, get = dma_rx_is_enabled, set = enable_dma_rx, clear = disable_dma_rx)]
    pub(super) RXDMAEN: bool,

    #[bits(1, rwc, get = dma_tx_is_enabled, set = enable_dma_tx, clear = disable_dma_tx)]
    pub(super) TXDMAEN: bool,

    #[bits(1, rwc, get = ss_is_enabled, set = enable_ss, clear = disable_ss)]
    pub(super) SSOE: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = get_frame_format_mode, set = set_frame_format_mode)]
    pub(super) FRF: FrameFormatMode,

    #[bits(
        1,
        rwc,
        get = error_interrupt_is_enabled,
        set = enable_error_interrupt,
        clear = disable_error_interrupt
    )]
    pub(super) ERRIE: bool,

    #[bits(
        1,
        rwc,
        get = rx_not_empty_interrupt_is_enabled,
        set = enable_rx_not_empty_interrupt,
        clear = disable_rx_not_empty_interrupt
    )]
    pub(super) RXNEIE: bool,

    #[bits(
        1,
        rwc,
        get = tx_empty_interrupt_is_enabled,
        set = enable_tx_empty_interrupt,
        clear = disable_tx_empty_interrupt
    )]
    pub(super) TXEIE: bool,

    #[bits(24)]
    __: u32,
}

#[register(u32)]
pub(super) struct StatusRegister {
    #[bits(1, r, get = rx_is_not_empty)]
    pub(super) RXNE: bool,

    #[bits(1, r, get = tx_is_empty)]
    pub(super) TXE: bool,

    #[bits(1, r, get = get_channel_side)]
    pub(super) CHSIDE: ChannelSide,

    #[bits(1, r, get = is_underrun)]
    pub(super) UDR: bool,

    #[bits(1, rc, get = is_crc_error, clear = clear_crc_error)]
    pub(super) CRCERR: bool,

    #[bits(1, r, get = is_mode_fault)]
    pub(super) MODF: bool,

    #[bits(1, r, get = is_overrun)]
    pub(super) OVR: bool,

    #[bits(1, r, get = is_busy)]
    pub(super) BSY: bool,

    #[bits(1, r, get = is_frame_format_error)]
    pub(super) FRE: bool,

    #[bits(23)]
    __: u32,
}

#[register(u32)]
pub struct DataRegister {
    #[bits(16, rw, get = read_data, set = write_data)]
    pub(super) DR: u16,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub struct CRCPolynomialRegister {
    #[bits(16, rw, get = get_crc_polynomial_value, set = set_crc_polynomial_value)]
    pub(super) CRCPOLY: u16,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub struct CRCRegister {
    #[bits(16, r, get = get_crc_value)]
    pub(super) CRC: u16,

    #[bits(16)]
    __: u32,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Slave = 0b0,
    Master = 0b1,
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
pub enum BaudRate {
    FpclkDiv2 = 0b000,
    FpclkDiv4 = 0b001,
    FpclkDiv8 = 0b010,
    FpclkDiv16 = 0b011,
    FpclkDiv32 = 0b100,
    FpclkDiv64 = 0b101,
    FpclkDiv128 = 0b110,
    FpclkDiv256 = 0b111,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    MSBFirst = 0b0,
    LSBFirst = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormatMode {
    Motorola = 0b0,
    TI = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFrameFormat {
    Format8Bit = 0b0,
    Format16Bit = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidirectionalMode {
    Receive = 0b0,
    Transmit = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelSide {
    Left = 0b0,
    Right = 0b1,
}
