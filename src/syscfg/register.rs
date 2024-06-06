use register::register;

use super::{ MemoryMappingMode, FlashBankMode, FMCMemorySwappingMode, EthernetPHYInterface };

#[register(u32)]
#[derive(Debug, Default)]
pub struct MemoryRemapRegister {
    #[bits(3, rw, get = get_memory_mapping_mode, set = set_memory_mapping_mode)]
    pub MEM_MODE: MemoryMappingMode,

    #[bits(5)]
    __: u32,

    #[bits(1, rw, get = get_flash_bank_mode, set = set_flash_bank_mode)]
    pub FB_MODE: FlashBankMode,

    #[bits(1)]
    __: u32,

    #[bits(2, rw, get = get_fmc_memory_mapping_swap, set = set_fmc_memory_mapping_swap)]
    pub SWP_FMC: FMCMemorySwappingMode,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct PeripheralModeConfigurationRegister {
    #[bits(15)]
    __: u32,

    /// TODO: rewrite to bitmask
    #[bits(3, rw)]
    pub ADCxDC2: u8,

    #[bits(4)]
    __: u32,

    #[bits(1, rw, get = get_ethernet_phy_interface, set = set_ethernet_phy_interface)]
    pub MIIRMIISEL: EthernetPHYInterface,

    #[bits(8)]
    __: u32,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct ExternalInterruptConfigurationRegister {
    #[bits(16, rw, get = get_exti_config, set = set_exti_config)]
    pub EXTI: u16,

    #[bits(16)]
    __: u32,
}

#[register(u32)]
#[derive(Debug, Default)]
pub struct CompensationCellControlRegister {
    #[bits(1, rw, get = compensation_cell_is_enabled, set = compensation_cell_enable)]
    pub CMPPD: bool,

    #[bits(7)]
    __: u32,

    #[bits(1, ro, get = compensation_cell_is_ready)]
    pub READY: bool,

    #[bits(23)]
    __: u32,
}
