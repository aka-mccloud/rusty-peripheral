use register::register;

use super::line::LineMask;

#[register(u32)]
pub struct InterruptMaskRegister {
    #[bits(23, rw, get = get_iterrupt_mask, set = set_interrupt_mask)]
    pub MR: LineMask,

    #[bits(9)]
    __: u32,
}

#[register(u32)]
pub struct EventMaskRegister {
    #[bits(23, rw, get = get_event_mask, set = set_event_mask)]
    pub MR: LineMask,

    #[bits(9)]
    __: u32,
}

#[register(u32)]
pub struct RisingTriggerSelectionRegister {
    #[bits(23, rw, get = get_rising_trigger_mask, set = set_rising_trigger_mask)]
    pub MR: LineMask,

    #[bits(9)]
    __: u32,
}

#[register(u32)]
pub struct FallingTriggerSelectionRegister {
    #[bits(23, rw, get = get_falling_trigger_mask, set = set_falling_trigger_mask)]
    pub MR: LineMask,

    #[bits(9)]
    __: u32,
}

#[register(u32)]
pub struct SoftwareInterruptEventRegister {
    #[bits(
        23,
        rw,
        get = get_software_interrupt_event_mask,
        set = set_software_interrupt_event_mask
    )]
    pub MR: LineMask,

    #[bits(9)]
    __: u32,
}

#[register(u32)]
pub struct PendingRegister {
    #[bits(23, rw, get = get_pending_interrupts, set = clear_pending_interrupts)]
    pub MR: LineMask,

    #[bits(9)]
    __: u32,
}
