use self::{register::{
    InterruptMaskRegister,
    EventMaskRegister,
    RisingTriggerSelectionRegister,
    FallingTriggerSelectionRegister,
    SoftwareInterruptEventRegister,
    PendingRegister,
}, line::LineMask};

pub mod register;
pub mod line;

pub struct EXTI {
    /// Interrupt Mask Register
    pub imr: InterruptMaskRegister,

    /// Event Mask Register
    pub emr: EventMaskRegister,

    /// Rising Trigger Selection Register
    pub rtsr: RisingTriggerSelectionRegister,

    /// Falling Trigger Selection Register
    pub ftsr: FallingTriggerSelectionRegister,

    /// Software Interrupt Event Register
    pub swier: SoftwareInterruptEventRegister,

    /// Pending Register
    pub pr: PendingRegister,
}

impl EXTI {
    pub fn get() -> &'static mut Self {
        let addr = 0x4001_3C00u32;

        unsafe {
            let ptr: *mut Self = addr as *mut Self;
            &mut *ptr
        }
    }

    #[inline]
    pub fn mask_interrupts_lines(&mut self, lines: impl Into<LineMask>) {
        let mask = lines.into();
        let lines = self.imr.get_iterrupt_mask();
        self.imr.set_interrupt_mask(lines & !mask);
    }

    #[inline]
    pub fn unmask_interrupts_lines(&mut self, lines: impl Into<LineMask>) {
        let mask = lines.into();
        let lines = self.imr.get_iterrupt_mask();
        self.imr.set_interrupt_mask(lines | mask);
    }

    #[inline]
    pub fn set_rising_trigger_lines(&mut self, lines: impl Into<LineMask>) {
        let mask = lines.into();
        let lines = self.rtsr.get_rising_trigger_mask();
        self.rtsr.set_rising_trigger_mask(lines | mask);
    }

    #[inline]
    pub fn reset_rising_trigger_lines(&mut self, lines: impl Into<LineMask>) {
        let mask = lines.into();
        let lines = self.rtsr.get_rising_trigger_mask();
        self.rtsr.set_rising_trigger_mask(lines & !mask);
    }

    #[inline]
    pub fn set_falling_trigger_lines(&mut self, lines: impl Into<LineMask>) {
        let mask = lines.into();
        let lines = self.ftsr.get_falling_trigger_mask();
        self.ftsr.set_falling_trigger_mask(lines | mask);
    }

    #[inline]
    pub fn reset_falling_trigger_lines(&mut self, lines: impl Into<LineMask>) {
        let mask = lines.into();
        let lines = self.ftsr.get_falling_trigger_mask();
        self.ftsr.set_falling_trigger_mask(lines & !mask);
    }
}
