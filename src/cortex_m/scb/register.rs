use register::register;

#[register(u32)]
pub(super) struct CPUIDBaseRegister {
    #[bits(4, ro)]
    pub(super) revision: u8,

    #[bits(12, ro)]
    pub(super) part_number: u16,

    #[bits(4)]
    __: u32,

    #[bits(4, ro)]
    pub(super) variant: u8,

    #[bits(8, ro)]
    pub(super) implementer: u8,
}

#[register(u32)]
pub(super) struct InterruptControlAndStateRegister {
    #[bits(9, rw)]
    pub(super) active_vector: u16,
    
    #[bits(2)]
    __: u32,

    #[bits(1, ro)]
    pub(super) return_to_base: bool,

    #[bits(7, ro)]
    pub(super) pending_vector: u16,

    #[bits(3)]
    __: u32,

    #[bits(1, ro, get = has_pending_interrupt)]
    pub(super) interrupt_pending_flag: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, wo, set = systick_exception_clear_pending_bit)]
    pub(super) PENDSTCLR: bool,

    #[bits(1, rw, get = systick_exception_is_pending, set = systick_exception_set_pending)]
    pub(super) PENDSTSET: bool,

    #[bits(1, wo, set = pendsv_exception_clear_pending_bit)]
    pub(super) PENDSVCLR: bool,

    #[bits(1, rw, get = pendsv_exception_is_pending, set = pendsv_exception_set_pending)]
    pub(super) PENDSVSET: bool,

    #[bits(2)]
    __: u32,

    #[bits(1, rw, get = nmi_exception_is_pending, set = nmi_exception_set_pending)]
    pub(super) NMIPENDSET: bool,
}