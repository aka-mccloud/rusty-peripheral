use register::register;

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

    #[bits(1, rw, get = peripheral_is_enabled, set = enable_peripheral)]
    pub(super) SPE: bool,

    #[bits(1, rw, get = get_frame_format, set = set_frame_format)]
    pub(super) SPE: FrameFormat,

    #[bits(1, rw, get = get_internal_slave_select, set = set_internal_slave_select)]
    pub(super) SSI: bool,

    #[bits(
        1,
        rw,
        get = software_slave_management_is_enabled,
        set = enable_software_slave_management
    )]
    pub(super) SSM: bool,

    #[bits(1, rw, get = receive_only_is_enabled, set = enable_receive_only)]
    pub(super) RXONLY: bool,

    #[bits(1, rw, get = get_data_frame_format, set = set_data_frame_format)]
    pub(super) DFF: DataFrameFormat,

    #[bits(1, rw, get = crc_transfer_is_next, set = set_next_transfer_crc)]
    pub(super) CRCNEXT: bool,

    #[bits(1, rw, get = hardware_crc_is_enabled, set = enable_hardware_crc)]
    pub(super) CRCEN: bool,

    #[bits(1, rw, get = get_bidirectional_mode, set = set_bidirectional_mode)]
    pub(super) BIDIOE: BidirectionalMode,

    #[bits(1, rw, get = bidirectinal_mode_is_enabled, set = enable_bidirectional_mode)]
    pub(super) BIDIMODE: bool,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
pub(super) struct ControlRegister2 {
    #[bits(1, rw, get = dma_rx_is_enabled, set = enable_dma_rx)]
    pub(super) RXDMAEN: bool,

    #[bits(1, rw, get = dma_tx_is_enabled, set = enable_dma_tx)]
    pub(super) TXDMAEN: bool,

    #[bits(1, rw, get = ss_is_enabled, set = enable_ss)]
    pub(super) SSOE: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = get_frame_format_mode, set = set_frame_format_mode)]
    pub(super) FRF: FrameFormatMode,

    #[bits(1, rw, get = error_interrupt_is_enabled, set = enable_error_interrupt)]
    pub(super) ERRIE: bool,

    #[bits(1, rw, get = rx_not_empty_interrupt_is_enabled, set = enable_rx_not_empty_interrupt)]
    pub(super) RXNEIE: bool,

    #[bits(1, rw, get = tx_empty_interrupt_is_enabled, set = enable_tx_empty_interrupt)]
    pub(super) TXEIE: bool,

    #[bits(24)]
    __: u32,
}

#[register(u32)]
pub(super) struct StatusRegister {
    #[bits(1, ro, get = rx_is_not_empty)]
    pub(super) RXNE: bool,

    #[bits(1, ro, get = tx_is_empty)]
    pub(super) TXE: bool,

    #[bits(1, ro, get = get_channel_side)]
    pub(super) CHSIDE: ChannelSide,

    #[bits(1, ro, get = is_underrun)]
    pub(super) UDR: bool,

    #[bits(1, roc, get = is_crc_error, clear = clear_crc_error)]
    pub(super) CRCERR: bool,

    #[bits(1, ro, get = is_mode_fault)]
    pub(super) MODF: bool,

    #[bits(1, ro, get = is_overrun)]
    pub(super) OVR: bool,

    #[bits(1, ro, get = is_busy)]
    pub(super) BSY: bool,

    #[bits(1, ro, get = is_frame_format_error)]
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
    #[bits(16, ro, get = get_crc_value)]
    pub(super) CRC: u16,

    #[bits(16)]
    __: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Slave = 0b0,
    Master = 0b1,
}

impl Mode {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::Slave,
            0b1 => Self::Master,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPhase {
    FirstClockTransition = 0b0,
    SecondClockTransition = 0b1,
}

impl ClockPhase {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::FirstClockTransition,
            0b1 => Self::SecondClockTransition,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPolarity {
    IdleLow = 0b0,
    IdleHigh = 0b1,
}

impl ClockPolarity {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::IdleLow,
            0b1 => Self::IdleHigh,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl BaudRate {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b000 => Self::FpclkDiv2,
            0b001 => Self::FpclkDiv4,
            0b010 => Self::FpclkDiv8,
            0b011 => Self::FpclkDiv16,
            0b100 => Self::FpclkDiv32,
            0b101 => Self::FpclkDiv64,
            0b110 => Self::FpclkDiv128,
            0b111 => Self::FpclkDiv256,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    MSBFirst = 0b0,
    LSBFirst = 0b1,
}

impl FrameFormat {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::MSBFirst,
            0b1 => Self::LSBFirst,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormatMode {
    Motorola = 0b0,
    TI = 0b1,
}

impl FrameFormatMode {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::Motorola,
            0b1 => Self::TI,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFrameFormat {
    Format8Bit = 0b0,
    Format16Bit = 0b1,
}

impl DataFrameFormat {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::Format8Bit,
            0b1 => Self::Format16Bit,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidirectionalMode {
    Receive = 0b0,
    Transmit = 0b1,
}

impl BidirectionalMode {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::Receive,
            0b1 => Self::Transmit,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelSide {
    Left = 0b0,
    Right = 0b1,
}

impl ChannelSide {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b0 => Self::Left,
            0b1 => Self::Right,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}
