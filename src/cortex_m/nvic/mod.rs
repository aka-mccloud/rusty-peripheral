use crate::peripheral;

pub struct NVIC {
    /// Interrupt Set-Enable Register
    pub iser: [u32; 8],

    __reserved0: [u32; 24],

    /// Interrupt Clear-Enable Register
    pub icer: [u32; 8],

    __reserved1: [u32; 24],

    /// Interrupt Set-Pending Register
    pub ispr: [u32; 8],

    __reserved2: [u32; 24],

    /// Interrupt Clear-Pending Register
    pub icpr: [u32; 8],

    __reserved3: [u32; 24],

    // Interrupt Active Bit Register
    pub iabr: [u32; 8],

    __reserved4: [u32; 56],

    /// Interrupt Priority Register
    pub ipr: [u32; 60],

    __reserved5: [u32; 644],

    pub stir: u32,
}

impl NVIC {
    pub fn irq_is_enabled(&mut self, irqn: usize) -> bool {
        (self.iser[irqn / 32] & (1 << irqn % 32)) != 0
    }

    pub fn irq_enable(&mut self, irqn: usize) {
        self.iser[irqn / 32] |= 1 << irqn % 32;
    }

    pub fn irq_disable(&mut self, irqn: usize) {
        self.icer[irqn / 32] |= 1 << irqn % 32;
    }

    pub fn irq_is_active(&mut self, irqn: usize) -> bool {
        (self.iabr[irqn / 32] & (1 << irqn % 32)) != 0
    }

    pub fn irq_set_pending(&mut self, irqn: usize) {
        self.ispr[irqn / 32] |= 1 << irqn % 32;
    }

    pub fn irq_clear_pending(&mut self, irqn: usize) {
        self.icpr[irqn / 32] |= 1 << irqn % 32;
    }

    pub fn irq_set_priority(&mut self, irqn: usize, priority: u8) {
        self.ipr[irqn / 4] |= (priority as u32) << (8 * (irqn % 4) + 4);
    }

    pub fn irq_trigger(&mut self, irqn: usize) {
        self.stir = irqn as _;
    }
}

pub fn nvic() -> &'static mut NVIC {
    peripheral(0xe000_e100)
}
