use core::{ marker::PhantomData, ops::{ Deref, DerefMut } };

pub mod nvic;
pub mod scb;

pub struct Peripherals {}

impl Peripherals {
    pub fn nvic() -> NVIC {
        NVIC::default()
    }

    pub fn scb() -> SCB {
        SCB::default()
    }
}

#[derive(Default)]
pub struct NVIC {
    _marker: PhantomData<*const ()>,
}

impl NVIC {
    pub const PTR: u32 = 0xe000_e100;
}

impl Deref for NVIC {
    type Target = nvic::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(Self::PTR as *const _) }
    }
}

impl DerefMut for NVIC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(Self::PTR as *mut _) }
    }
}

#[derive(Default)]
pub struct SCB {
    _marker: PhantomData<*const ()>,
}

impl SCB {
    pub const PTR: u32 = 0xe000_ed00;
}

impl Deref for SCB {
    type Target = scb::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(Self::PTR as *const _) }
    }
}

impl DerefMut for SCB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(Self::PTR as *mut _) }
    }
}
