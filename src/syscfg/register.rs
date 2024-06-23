use register::{ field::derive::RegisterField, register };

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
    #[bits(1, rwc, get = compensation_cell_is_enabled, set = compensation_cell_enable, clear = compensation_cell_disable)]
    pub CMPPD: bool,

    #[bits(7)]
    __: u32,

    #[bits(1, r, get = compensation_cell_is_ready)]
    pub READY: bool,

    #[bits(23)]
    __: u32,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryMappingMode {
    /// Main Flash
    MainFlash = 0b000,

    /// System Flash
    SystemFlash = 0b001,

    /// FMC Bank1 (NOR/PSRAM 1 and 2)
    FMCBank1 = 0b010,

    /// Embedded SRAM (SRAM1)
    EmbeddedSRAM = 0b011,

    /// FMC/SDRAM Bank 1
    SDRAMBank1 = 0b100,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashBankMode {
    /// Flash Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000) and
    /// Flash Bank 2 is mapped at 0x0810 0000 (and aliased at 0x0010 0000)
    Bank1 = 0b0,

    /// Flash Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000) and
    /// Flash Bank 1 is mapped at 0x0810 0000 (and aliased at 0x0010 0000)
    Bank2 = 0b1,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FMCMemorySwappingMode {
    /// No FMC memory mapping swap
    None = 0b00,

    /// SDRAM banks and NAND Bank 2/PCCARD mapping are swapped.
    ///
    /// SDRAM Bank 1 and 2 are mapped at NAND Bank 2 (0x8000 0000) and PCCARD Bank (0x9000 0000) address, respectively.
    ///
    /// NAND Bank 2 and PCCARD Bank are mapped at 0xC000 0000 and 0xD000 0000, respectively.
    Swap = 0b01,
}

#[derive(RegisterField, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthernetPHYInterface {
    /// MII interface
    MII = 0b0,

    /// RMII PHY interface
    RMII = 0b1,
}
