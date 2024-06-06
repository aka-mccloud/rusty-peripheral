use register::register;

use crate::{ State, gpio::port::PortMask };

use super::{
    SystemClockSource,
    PLLClockSource,
    MCOClockSource,
    I2SClockSource,
    SAIClockSource,
    RTCClockSource,
    AHBPrescaler,
    APBPrescaler,
    MCOPrescaler,
    TimerPrescaler,
    SpreadSelect,
};

#[register(u32)]
pub struct ClockControlRegister {
    #[bits(1, rw, get = hsi_get_state, set = hsi_set)]
    pub HSION: State,

    #[bits(1, ro, get = hsi_is_ready)]
    pub HSIRDY: bool,

    #[bits(1)]
    __: u8,

    #[bits(
        5,
        rwc,
        get = hsi_get_trimming_value,
        set = hsi_set_trimming_value,
        clear = hsi_clear_trimming_value
    )]
    pub HSITRIM: u8,

    #[bits(8, ro, get = hsi_get_calibration_value)]
    pub HSICAL: u8,

    #[bits(1, rw, get = hse_get_state, set = hse_set)]
    pub HSEON: State,

    #[bits(1, ro, get = hse_is_ready)]
    pub HSERDY: bool,

    #[bits(1, rw, get = hse_bypass_get_state, set = hse_bypass_set)]
    pub HSEBYP: State,

    #[bits(1, rw, get = css_get_state, set = css_set)]
    pub CSSON: State,

    #[bits(4)]
    __: u8,

    #[bits(1, rw, get = pll_get_state, set = pll_set)]
    pub PLLON: State,

    #[bits(1, ro, get = pll_is_ready)]
    pub PLLRDY: bool,

    #[bits(1, rw, get = pll_i2s_get_state, set = pll_i2s_set)]
    pub PLLI2SON: State,

    #[bits(1, ro, get = pll_i2s_is_ready)]
    pub PLLI2SRDY: bool,

    #[bits(1, rw, get = pll_sai_get_state, set = pll_sai_set)]
    pub PLLSAION: State,

    #[bits(1, ro, get = pll_sai_is_ready)]
    pub PLLSAIRDY: bool,

    #[bits(2)]
    __: u8,
}

#[register(u32)]
pub struct PLLConfigurationRegister {
    #[bits(6, rw, get = pll_get_division_factor, set = pll_set_division_factor)]
    pub PLLM: u32,

    #[bits(9, rw, get = pll_get_mutiplication_factor, set = pll_set_multiplication_factor)]
    pub PLLN: u32,

    #[bits(1)]
    __: u32,

    #[bits(2, rw, get = pll_get_sysclock_division_factor, set = pll_set_sysclock_difision_factor)]
    pub PLLP: PLLSysClockDivisionFactor,

    #[bits(4)]
    __: u32,

    #[bits(1, rw, get = pll_get_clock_source, set = pll_set_clock_source)]
    pub PLLSRC: PLLClockSource,

    #[bits(1)]
    __: u32,

    #[bits(
        4,
        rw,
        get = pll_get_usb_sdio_rng_division_factor,
        set = pll_set_usb_sdio_rng_division_factor
    )]
    pub PLLQ: u32,

    #[bits(4)]
    __: u32,
}

#[register(u32)]
pub struct ClockConfigurationRegister {
    #[bits(2, rw, get = sysclock_get_clock_source, set = sysclock_set_clock_source)]
    pub SW: SystemClockSource,

    #[bits(2, ro, get = sysclock_get_used_clock_source)]
    pub SWS: SystemClockSource,

    #[bits(4, rw, get = ahb_get_prescaler, set = ahb_set_prescaler)]
    pub HPRE: AHBPrescaler,

    #[bits(2)]
    __: u32,

    #[bits(3, rw, get = apb1_get_prescaler, set = apb1_set_prescaler)]
    pub PPRE1: APBPrescaler,

    #[bits(3, rw, get = apb2_get_prescaler, set = apb2_set_prescaler)]
    pub PPRE2: APBPrescaler,

    #[bits(5, rw, get = rtc_get_division_factor, set = rtc_set_division_factor)]
    pub RTCPRE: u8,

    #[bits(2, rw, get = mco1_get_clock_source, set = mco1_set_clock_cource)]
    pub MCO1: MCOClockSource,

    #[bits(1, rw, get = i2s_get_clock_source, set = i2s_set_clock_source)]
    pub I2SSRC: I2SClockSource,

    #[bits(3, rw, get = mco1_get_prescaler, set = mco1_set_prescaler)]
    pub MCO1PRE: MCOPrescaler,

    #[bits(3, rw, get = mco2_get_prescaler, set = mco2_set_prescaler)]
    pub MCO2PRE: MCOPrescaler,

    #[bits(2, rw, get = mco2_get_clock_source, set = mco2_set_clock_cource)]
    pub MCO2: MCOClockSource,
}

#[register(u32)]
pub struct ClockInterruptRegister {
    #[bits(1, ro, get = lsi_is_ready_inerrupt_set)]
    pub LSIRDYF: bool,

    #[bits(1, ro, get = lse_is_ready_inerrupt_set)]
    pub LSERDYF: bool,

    #[bits(1, ro, get = hsi_is_ready_inerrupt_set)]
    pub HSIRDYF: bool,

    #[bits(1, ro, get = hse_is_ready_inerrupt_set)]
    pub HSERDYF: bool,

    #[bits(1, ro, get = pll_is_ready_inerrupt_set)]
    pub PLLRDYF: bool,

    #[bits(1, ro, get = pll_i2s_is_ready_inerrupt_set)]
    pub PLLI2SRDYF: bool,

    #[bits(1, ro, get = pll_sai_is_ready_inerrupt_set)]
    pub PLLSAIRDYF: bool,

    #[bits(1, ro, get = css_is_hse_failure_inerrupt_set)]
    pub CSSF: bool,

    #[bits(1, rw, get = lsi_is_ready_inerrupt_enabled, set = lsi_enable_ready_iterrupt)]
    pub LSIRDYIE: bool,

    #[bits(1, rw, get = lse_is_ready_inerrupt_enabled, set = lse_enable_ready_iterrupt)]
    pub LSERDYIE: bool,

    #[bits(1, rw, get = hsi_is_ready_inerrupt_enabled, set = hsi_enable_ready_iterrupt)]
    pub HSIRDYIE: bool,

    #[bits(1, rw, get = hse_is_ready_inerrupt_enabled, set = hse_enable_ready_iterrupt)]
    pub HSERDYIE: bool,

    #[bits(1, rw, get = pll_is_ready_inerrupt_enabled, set = pll_enable_ready_iterrupt)]
    pub PLLRDYIE: bool,

    #[bits(1, rw, get = pll_i2s_is_ready_inerrupt_enabled, set = pll_i2s_enable_ready_iterrupt)]
    pub PLLI2SRDYIE: bool,

    #[bits(1, rw, get = pll_sai_is_ready_inerrupt_enabled, set = pll_sai_enable_ready_iterrupt)]
    pub PLLSAIRDYIE: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, wo, set = lsi_clear_ready_iterrupt)]
    pub LSIRDYC: bool,

    #[bits(1, wo, set = lse_clear_ready_iterrupt)]
    pub LSERDYC: bool,

    #[bits(1, wo, set = hsi_clear_ready_iterrupt)]
    pub HSIRDYC: bool,

    #[bits(1, wo, set = hse_clear_ready_iterrupt)]
    pub HSERDYC: bool,

    #[bits(1, wo, set = pll_clear_ready_iterrupt)]
    pub PLLRDYC: bool,

    #[bits(1, wo, set = pll_i2s_clear_ready_iterrupt)]
    pub PLLI2SRDYC: bool,

    #[bits(1, wo, set = pll_sai_clear_ready_iterrupt)]
    pub PLLSAIRDYC: bool,

    #[bits(1, wo, set = css_clear_hse_failure_iterrupt)]
    pub CSSC: bool,

    #[bits(8)]
    __: u32,
}

#[register(u32)]
pub struct AHB1PeripheralResetRegister {
    #[bits(11, rwc, get = gpio_get_reset_state, set = gpio_reset, clear = gpio_reset_clear)]
    pub GPIORST: PortMask,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = crc_is_in_reset_state, set = crc_reset)]
    pub CRCRST: bool,

    #[bits(8)]
    __: u32,

    #[bits(1, rw, get = dma1_is_in_reset_state, set = dma1_reset)]
    pub DMA1RST: bool,

    #[bits(1, rw, get = dma2_is_in_reset_state, set = dma2_reset)]
    pub DMA2RST: bool,

    #[bits(1, rw, get = dma3_is_in_reset_state, set = dma3_reset)]
    pub DMA3RST: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = eth_mac_is_in_reset_state, set = eth_mac_reset)]
    pub ETHMACRST: bool,

    #[bits(3)]
    __: u32,

    #[bits(1, rw, get = usb_otg_hs_is_in_reset_state, set = usb_otg_hs_reset)]
    pub OTGHSRST: bool,

    #[bits(2)]
    __: u32,
}

#[register(u32)]
pub struct AHB2PeripheralResetRegister {
    #[bits(1, rw, get = dcmi_is_in_reset_state, set = dcmi_reset)]
    pub DCMIRST: bool,

    #[bits(3)]
    __: u32,

    #[bits(1, rw, get = crypt_is_in_reset_state, set = crypt_reset)]
    pub CRYPRST: bool,

    #[bits(1, rw, get = hash_is_in_reset_state, set = hash_reset)]
    pub HASHRST: bool,

    #[bits(1, rw, get = rng_is_in_reset_state, set = rng_reset)]
    pub RNGRST: bool,

    #[bits(1, rw, get = usb_otg_fs_is_in_reset_state, set = usb_otg_fs_reset)]
    pub OTGFSRST: bool,

    #[bits(24)]
    __: u32,
}

#[register(u32)]
pub struct AHB3PeripheralResetRegister {
    #[bits(1, rw, get = fmc_is_in_reset_state, set = fmc_reset)]
    pub FMCRST: bool,

    #[bits(31)]
    __: u32,
}

#[register(u32)]
pub struct APB1PeripheralResetRegister {
    #[bits(1, rw, get = timer2_is_in_reset_state, set = timer2_reset)]
    pub TIM2RST: bool,

    #[bits(1, rw, get = timer3_is_in_reset_state, set = timer3_reset)]
    pub TIM3RST: bool,

    #[bits(1, rw, get = timer4_is_in_reset_state, set = timer4_reset)]
    pub TIM4RST: bool,

    #[bits(1, rw, get = timer5_is_in_reset_state, set = timer5_reset)]
    pub TIM5RST: bool,

    #[bits(1, rw, get = timer6_is_in_reset_state, set = timer6_reset)]
    pub TIM6RST: bool,

    #[bits(1, rw, get = timer7_is_in_reset_state, set = timer7_reset)]
    pub TIM7RST: bool,

    #[bits(1, rw, get = timer12_is_in_reset_state, set = timer12_reset)]
    pub TIM12RST: bool,

    #[bits(1, rw, get = timer13_is_in_reset_state, set = timer13_reset)]
    pub TIM13RST: bool,

    #[bits(1, rw, get = timer14_is_in_reset_state, set = timer14_reset)]
    pub TIM14RST: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = wwdg_is_in_reset_state, set = wwdg_reset)]
    pub WWDGRST: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = spi2_is_in_reset_state, set = spi2_reset)]
    pub SPI2RST: bool,

    #[bits(1, rw, get = spi3_is_in_reset_state, set = spi3_reset)]
    pub SPI3RST: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = usart2_is_in_reset_state, set = usart2_reset)]
    pub USART2RST: bool,

    #[bits(1, rw, get = usart3_is_in_reset_state, set = usart3_reset)]
    pub USART3RST: bool,

    #[bits(1, rw, get = uart4_is_in_reset_state, set = uart4_reset)]
    pub UART4RST: bool,

    #[bits(1, rw, get = uart5_is_in_reset_state, set = uart5_reset)]
    pub UART5RST: bool,

    #[bits(1, rw, get = i2c1_is_in_reset_state, set = i2c1_reset)]
    pub I2C1RST: bool,

    #[bits(1, rw, get = i2c2_is_in_reset_state, set = i2c2_reset)]
    pub I2C2RST: bool,

    #[bits(1, rw, get = i2c3_is_in_reset_state, set = i2c3_reset)]
    pub I2C3RST: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = can1_is_in_reset_state, set = can1_reset)]
    pub CAN1RST: bool,

    #[bits(1, rw, get = can2_is_in_reset_state, set = can2_reset)]
    pub CAN2RST: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = pwr_is_in_reset_state, set = pwr_reset)]
    pub PWRRST: bool,

    #[bits(1, rw, get = dac_is_in_reset_state, set = dac_reset)]
    pub DACRST: bool,

    #[bits(1, rw, get = uart7_is_in_reset_state, set = uart7_reset)]
    pub UART7RST: bool,

    #[bits(1, rw, get = uart8_is_in_reset_state, set = uart8_reset)]
    pub UART8RST: bool,
}

#[register(u32)]
pub struct APB2PeripheralResetRegister {
    #[bits(1, rw, get = timer1_is_in_reset_state, set = timer1_reset)]
    pub TIM1RST: bool,

    #[bits(1, rw, get = timer8_is_in_reset_state, set = timer8_reset)]
    pub TIM8RST: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = usart1_is_in_reset_state, set = usart1_reset)]
    pub USART1RST: bool,

    #[bits(1, rw, get = usart6_is_in_reset_state, set = usart6_reset)]
    pub USART6RST: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = adc_is_in_reset_state, set = adc_reset)]
    pub ADCRST: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = sdio_is_in_reset_state, set = sdio_reset)]
    pub SDIORST: bool,

    #[bits(1, rw, get = spi1_is_in_reset_state, set = spi1_reset)]
    pub SPI1RST: bool,

    #[bits(1, rw, get = spi4_is_in_reset_state, set = spi4_reset)]
    pub SPI4RST: bool,

    #[bits(1, rw, get = syscfg_is_in_reset_state, set = syscfg_reset)]
    pub SYSCFGRST: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = timer9_is_in_reset_state, set = timer9_reset)]
    pub TIM9RST: bool,

    #[bits(1, rw, get = timer10_is_in_reset_state, set = timer10_reset)]
    pub TIM10RST: bool,

    #[bits(1, rw, get = timer11_is_in_reset_state, set = timer11_reset)]
    pub TIM11RST: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = spi5_is_in_reset_state, set = spi5_reset)]
    pub SPI5RST: bool,

    #[bits(1, rw, get = spi6_is_in_reset_state, set = spi6_reset)]
    pub SPI6RST: bool,

    #[bits(1, rw, get = sai1_is_in_reset_state, set = sai1_reset)]
    pub SAI1RST: bool,

    #[bits(3)]
    __: u32,

    #[bits(1, rw, get = ltdc_is_in_reset_state, set = ltdc_reset)]
    pub LTDCRST: bool,

    #[bits(5)]
    __: u32,
}

#[register(u32)]
pub struct AHB1PeripheralClockRegister {
    #[bits(11, rw, get = gpio_get_enabled, set = gpio_enable)]
    pub GPIOEN: PortMask,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = crc_is_enabled, set = crc_enable)]
    pub CRCEN: bool,

    #[bits(5)]
    __: u32,

    #[bits(1, rw, get = backup_sram_is_enabled, set = backup_sram_enable)]
    pub BKPSRAMEN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = ccm_data_ram_is_enabled, set = ccm_data_ram_enable)]
    pub CCMDATARAMEN: bool,

    #[bits(1, rw, get = dma1_is_enabled, set = dma1_enable)]
    pub DMA1EN: bool,

    #[bits(1, rw, get = dma2_is_enabled, set = dma2_enable)]
    pub DMA2EN: bool,

    #[bits(1, rw, get = dma2d_is_enabled, set = dma2d_enable)]
    pub DMA2DEN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = eth_mac_is_enabled, set = eth_mac_enable)]
    pub ETHMACEN: bool,

    #[bits(1, rw, get = eth_mac_tx_is_enabled, set = eth_mac_tx_enable)]
    pub ETHMACTXEN: bool,

    #[bits(1, rw, get = eth_mac_rx_is_enabled, set = eth_mac_rx_enable)]
    pub ETHMACRXEN: bool,

    #[bits(1, rw, get = eth_mac_ptp_is_enabled, set = eth_mac_ptp_enable)]
    pub ETHMACPTPEN: bool,

    #[bits(1, rw, get = usb_otg_hs_is_enabled, set = usb_otg_hs_enable)]
    pub OTGHSEN: bool,

    #[bits(1, rw, get = usb_otg_hs_ulpi_is_enabled, set = usb_otg_hs_ulpi_enable)]
    pub OTGHSULPIEN: bool,

    #[bits(1)]
    __: u32,
}

#[register(u32)]
pub struct AHB2PeripheralClockRegister {
    #[bits(1, rw, get = dcmi_is_enabled, set = dcmi_enable)]
    pub DCMIEN: bool,

    #[bits(3)]
    __: u32,

    #[bits(1, rw, get = crypt_is_enabled, set = crypt_enable)]
    pub CRYPEN: bool,

    #[bits(1, rw, get = hash_is_enabled, set = hash_enable)]
    pub HASHEN: bool,

    #[bits(1, rw, get = rng_is_enabled, set = rng_enable)]
    pub RNGEN: bool,

    #[bits(1, rw, get = usb_otg_fs_is_enabled, set = usb_otg_fs_enable)]
    pub OTGFSEN: bool,

    #[bits(24)]
    __: u32,
}

#[register(u32)]
pub struct AHB3PeripheralClockRegister {
    #[bits(1, rw, get = fmc_is_enabled, set = fmc_enable)]
    pub FMCEN: bool,

    #[bits(31)]
    __: u32,
}

#[register(u32)]
pub struct APB1PeripheralClockRegister {
    #[bits(1, rw, get = timer2_is_enabled, set = timer2_enable)]
    pub TIM2EN: bool,

    #[bits(1, rw, get = timer3_is_enabled, set = timer3_enable)]
    pub TIM3EN: bool,

    #[bits(1, rw, get = timer4_is_enabled, set = timer4_enable)]
    pub TIM4EN: bool,

    #[bits(1, rw, get = timer5_is_enabled, set = timer5_enable)]
    pub TIM5EN: bool,

    #[bits(1, rw, get = timer6_is_enabled, set = timer6_enable)]
    pub TIM6EN: bool,

    #[bits(1, rw, get = timer7_is_enabled, set = timer7_enable)]
    pub TIM7EN: bool,

    #[bits(1, rw, get = timer12_is_enabled, set = timer12_enable)]
    pub TIM12EN: bool,

    #[bits(1, rw, get = timer13_is_enabled, set = timer13_enable)]
    pub TIM13EN: bool,

    #[bits(1, rw, get = timer14_is_enabled, set = timer14_enable)]
    pub TIM14EN: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = wwdg_is_enabled, set = wwdg_enable)]
    pub WWDGEN: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = spi2_is_enabled, set = spi2_enable)]
    pub SPI2EN: bool,

    #[bits(1, rw, get = spi3_is_enabled, set = spi3_enable)]
    pub SPI3EN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = usart2_is_enabled, set = usart2_enable)]
    pub USART2EN: bool,

    #[bits(1, rw, get = usart3_is_enabled, set = usart3_enable)]
    pub USART3EN: bool,

    #[bits(1, rw, get = uart4_is_enabled, set = uart4_enable)]
    pub UART4EN: bool,

    #[bits(1, rw, get = uart5_is_enabled, set = uart5_enable)]
    pub UART5EN: bool,

    #[bits(1, rw, get = i2c1_is_enabled, set = i2c1_enable)]
    pub I2C1EN: bool,

    #[bits(1, rw, get = i2c2_is_enabled, set = i2c2_enable)]
    pub I2C2EN: bool,

    #[bits(1, rw, get = i2c3_is_enabled, set = i2c3_enable)]
    pub I2C3EN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = can1_is_enabled, set = can1_enable)]
    pub CAN1EN: bool,

    #[bits(1, rw, get = can2_is_enabled, set = can2_enable)]
    pub CAN2EN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = pwr_is_enabled, set = pwr_enable)]
    pub PWREN: bool,

    #[bits(1, rw, get = dac_is_enabled, set = dac_enable)]
    pub DACEN: bool,

    #[bits(1, rw, get = uart7_is_enabled, set = uart7_enable)]
    pub UART7EN: bool,

    #[bits(1, rw, get = uart8_is_enabled, set = uart8_enable)]
    pub UART8EN: bool,
}

#[register(u32)]
pub struct APB2PeripheralClockRegister {
    #[bits(1, rw, get = timer1_is_enabled, set = timer1_enable)]
    pub TIM1EN: bool,

    #[bits(1, rw, get = timer8_is_enabled, set = timer8_enable)]
    pub TIM8EN: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = usart1_is_enabled, set = usart1_enable)]
    pub USART1EN: bool,

    #[bits(1, rw, get = usart6_is_enabled, set = usart6_enable)]
    pub USART6EN: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = adc1_is_enabled, set = adc1_enable)]
    pub ADC1EN: bool,

    #[bits(1, rw, get = adc2_is_enabled, set = adc2_enable)]
    pub ADC2EN: bool,

    #[bits(1, rw, get = adc3_is_enabled, set = adc3_enable)]
    pub ADC3EN: bool,

    #[bits(1, rw, get = sdio_is_enabled, set = sdio_enable)]
    pub SDIOEN: bool,

    #[bits(1, rw, get = spi1_is_enabled, set = spi1_enable)]
    pub SPI1EN: bool,

    #[bits(1, rw, get = spi4_is_enabled, set = spi4_enable)]
    pub SPI4EN: bool,

    #[bits(1, rw, get = syscfg_is_enabled, set = syscfg_enable)]
    pub SYSCFGEN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = timer9_is_enabled, set = timer9_enable)]
    pub TIM9EN: bool,

    #[bits(1, rw, get = timer10_is_enabled, set = timer10_enable)]
    pub TIM10EN: bool,

    #[bits(1, rw, get = timer11_is_enabled, set = timer11_enable)]
    pub TIM11EN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = spi5_is_enabled, set = spi5_enable)]
    pub SPI5EN: bool,

    #[bits(1, rw, get = spi6_is_enabled, set = spi6_enable)]
    pub SPI6EN: bool,

    #[bits(1, rw, get = sai1_is_enabled, set = sai1_enable)]
    pub SAI1EN: bool,

    #[bits(3)]
    __: u32,

    #[bits(1, rw, get = ltdc_is_enabled, set = ltdc_enable)]
    pub LTDCEN: bool,

    #[bits(5)]
    __: u32,
}

#[register(u32)]
pub struct AHB1PeripheralClockLowPowerModeRegister {
    #[bits(1, rw, get = gpioa_is_enabled, set = gpioa_enable)]
    pub GPIOALPEN: bool,

    #[bits(1, rw, get = gpiob_is_enabled, set = gpiob_enable)]
    pub GPIOBLPEN: bool,

    #[bits(1, rw, get = gpioc_is_enabled, set = gpioc_enable)]
    pub GPIOCLPEN: bool,

    #[bits(1, rw, get = gpiod_is_enabled, set = gpiod_enable)]
    pub GPIODLPEN: bool,

    #[bits(1, rw, get = gpioe_is_enabled, set = gpioe_enable)]
    pub GPIOELPEN: bool,

    #[bits(1, rw, get = gpiof_is_enabled, set = gpiof_enable)]
    pub GPIOFLPEN: bool,

    #[bits(1, rw, get = gpiog_is_enabled, set = gpiog_enable)]
    pub GPIOGLPEN: bool,

    #[bits(1, rw, get = gpioh_is_enabled, set = gpioh_enable)]
    pub GPIOHLPEN: bool,

    #[bits(1, rw, get = gpioi_is_enabled, set = gpioi_enable)]
    pub GPIOILPEN: bool,

    #[bits(1, rw, get = gpioj_is_enabled, set = gpioj_enable)]
    pub GPIOJLPEN: bool,

    #[bits(1, rw, get = gpiok_is_enabled, set = gpiok_enable)]
    pub GPIOKLPEN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = crc_is_enabled, set = crc_enable)]
    pub CRCLPEN: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = flash_is_enabled, set = flash_enable)]
    pub FLITFLPEN: bool,

    #[bits(1, rw, get = sram1_is_enabled, set = sram1_enable)]
    pub SRAM1LPEN: bool,

    #[bits(1, rw, get = sram2_is_enabled, set = sram2_enable)]
    pub SRAM2LPEN: bool,

    #[bits(1, rw, get = backup_sram_is_enabled, set = backup_sram_enable)]
    pub BKPSRAMLPEN: bool,

    #[bits(1, rw, get = sram3_is_enabled, set = sram3_enable)]
    pub SRAM3LPEN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = dma1_is_enabled, set = dma1_enable)]
    pub DMA1LPEN: bool,

    #[bits(1, rw, get = dma2_is_enabled, set = dma2_enable)]
    pub DMA2LPEN: bool,

    #[bits(1, rw, get = dma2d_is_enabled, set = dma2d_enable)]
    pub DMA2DLPEN: bool,

    #[bits(1)]
    __: u32,

    #[bits(1, rw, get = eth_mac_is_enabled, set = eth_mac_enable)]
    pub ETHMACLPEN: bool,

    #[bits(1, rw, get = eth_mac_tx_is_enabled, set = eth_mac_tx_enable)]
    pub ETHMACTXLPEN: bool,

    #[bits(1, rw, get = eth_mac_rx_is_enabled, set = eth_mac_rx_enable)]
    pub ETHMACRXLPEN: bool,

    #[bits(1, rw, get = eth_mac_ptp_is_enabled, set = eth_mac_ptp_enable)]
    pub ETHMACPTPLPEN: bool,

    #[bits(1, rw, get = usb_otg_hs_is_enabled, set = usb_otg_hs_enable)]
    pub OTGHSLPEN: bool,

    #[bits(1, rw, get = usb_otg_hs_ulpi_is_enabled, set = usb_otg_hs_ulpi_enable)]
    pub OTGHSULPILPEN: bool,

    #[bits(1)]
    __: u32,
}

#[register(u32)]
pub struct BackupDomainControlRegister {
    #[bits(1, rw, get = lse_get_state, set = lse_set)]
    pub LSEON: State,

    #[bits(1, ro, get = lse_is_ready)]
    pub LSERDY: bool,

    #[bits(1, rw, get = lse_bypass_get_state, set = lse_bypass_set)]
    pub LSEBYP: State,

    #[bits(5)]
    __: u32,

    #[bits(2, rw, get = rtc_get_clock_source, set = rtc_set_clock_source)]
    pub RTCSEL: RTCClockSource,

    #[bits(5)]
    __: u32,

    #[bits(1, rw, get = rtc_is_enabled, set = rtc_enable)]
    pub RTCEN: bool,

    #[bits(1, rw, get = backup_domain_is_in_reset_state, set = backup_domain_reset)]
    pub BDRST: bool,

    #[bits(15)]
    __: u32,
}

#[register(u32)]
pub struct ClockControlAndStatusRegister {
    #[bits(1, rw, get = lsi_get_state, set = lsi_set)]
    pub LSION: State,

    #[bits(1, ro, get = lsi_is_ready)]
    pub LSIRDY: bool,

    #[bits(22)]
    __: u32,

    #[bits(1, w0, set = remove_reset_flag)]
    pub RMVF: bool,

    #[bits(1, ro, get = is_bor_reset_occured)]
    pub BORRSTF: bool,

    #[bits(1, ro, get = is_pin_reset_occured)]
    pub PINRSTF: bool,

    #[bits(1, ro, get = is_por_reset_occured)]
    pub PORRSTF: bool,

    #[bits(1, ro, get = is_sofware_reset_occured)]
    pub SFTRSTF: bool,

    #[bits(1, ro, get = is_iwdg_reset_occured)]
    pub IWDGRSTF: bool,

    #[bits(1, ro, get = is_wwdg_reset_occured)]
    pub WWDGRSTF: bool,

    #[bits(1, ro, get = is_low_power_reset_occured)]
    pub LPWRRSTF: bool,
}

#[register(u32)]
pub struct SpreadSpectrumClockGenerationRegister {
    #[bits(13, rw, get = get_modulation_period, set = set_modulation_period)]
    pub MODPER: u16,

    #[bits(15, rw, get = get_increment_step, set = set_increment_step)]
    pub INCSTEP: u16,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = get_spread, set = set_spread)]
    pub SPREADSEL: SpreadSelect,

    #[bits(
        1,
        rw,
        get = is_spread_spectrum_modulation_enabled,
        set = enable_spread_spectrum_modulation
    )]
    pub SSCGEN: bool,
}

#[register(u32)]
pub struct PLLI2SConfigurationRegister {
    #[bits(6)]
    __: u32,

    #[bits(
        9,
        rw,
        get = pll_i2s_vco_get_multiplication_factor,
        set = pll_i2s_vco_set_multiplication_factor
    )]
    pub PLLI2SN: u16,

    #[bits(9)]
    __: u32,

    #[bits(4, rw, get = pll_i2s_sai1_get_division_factor, set = pll_i2s_sai1_set_division_factor)]
    pub PLLI2SQ: u8,

    #[bits(3, rw, get = pll_i2s_i2s_get_division_factor, set = pll_i2s_i2s_set_division_factor)]
    pub PLLI2SR: u8,

    #[bits(1)]
    __: u32,
}

#[register(u32)]
pub struct PLLSAIConfigurationRegister {
    #[bits(6)]
    __: u32,

    #[bits(
        9,
        rw,
        get = pll_sai_vco_get_multiplication_factor,
        set = pll_sai_vco_set_multiplication_factor
    )]
    pub PLLSAIN: u16,

    #[bits(9)]
    __: u32,

    #[bits(4, rw, get = pll_sai_sai1_get_division_factor, set = pll_sai_sai1_set_division_factor)]
    pub PLLSAIQ: u8,

    #[bits(3, rw, get = pll_sai_lcd_get_division_factor, set = pll_sai_lcd_set_division_factor)]
    pub PLLSAIR: u8,

    #[bits(1)]
    __: u32,
}

#[register(u32)]
pub struct DedicatedClockConfigurationRegister {
    #[bits(5, rw, get = pll_i2s_sai_get_division_factor, set = pll_i2s_sai_set_division_factor)]
    pub PLLI2SDIVQ: u8,

    #[bits(3)]
    __: u32,

    #[bits(9, rw, get = pll_sai_sai1_get_division_factor, set = pll_sai_sai1_set_division_factor)]
    pub PLLSAIDIVQ: u16,

    #[bits(3)]
    __: u32,

    #[bits(2, rw, get = sai1a_get_clock_source, set = sai1a_set_clock_source)]
    pub SAI1ASRC: SAIClockSource,

    #[bits(2, rw, get = sai1b_get_clock_source, set = sai1b_set_clock_source)]
    pub SAI1BSRC: SAIClockSource,

    #[bits(1, rw, get = timers_get_clock_prescaler, set = timers_set_clock_prescaler)]
    pub TIMPRE: TimerPrescaler,

    #[bits(7)]
    __: u32,
}

pub enum PLLSysClockDivisionFactor {
    DividedBy2 = 0b00,
    DividedBy4 = 0b01,
    DividedBy6 = 0b10,
    DividedBy8 = 0b11,
}

impl PLLSysClockDivisionFactor {
    pub fn from_bits(val: u32) -> Self {
        match val {
            0b00 => Self::DividedBy2,
            0b01 => Self::DividedBy4,
            0b10 => Self::DividedBy6,
            0b11 => Self::DividedBy8,
            _ => panic!(),
        }
    }

    pub fn into_bits(val: Self) -> u32 {
        val as _
    }
}
