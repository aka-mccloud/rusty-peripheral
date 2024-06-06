use core::ops::{ BitOr, BitAnd, BitOrAssign, BitAndAssign, BitXor, BitXorAssign, Not };

use crate::exti::line::LineMask;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pin {
    PIN0 = 0b0000_0000_0000_0001,
    PIN1 = 0b0000_0000_0000_0010,
    PIN2 = 0b0000_0000_0000_0100,
    PIN3 = 0b0000_0000_0000_1000,
    PIN4 = 0b0000_0000_0001_0000,
    PIN5 = 0b0000_0000_0010_0000,
    PIN6 = 0b0000_0000_0100_0000,
    PIN7 = 0b0000_0000_1000_0000,
    PIN8 = 0b0000_0001_0000_0000,
    PIN9 = 0b0000_0010_0000_0000,
    PIN10 = 0b0000_0100_0000_0000,
    PIN11 = 0b0000_1000_0000_0000,
    PIN12 = 0b0001_0000_0000_0000,
    PIN13 = 0b0010_0000_0000_0000,
    PIN14 = 0b0100_0000_0000_0000,
    PIN15 = 0b1000_0000_0000_0000,
}

impl Not for Pin {
    type Output = PinMask;

    #[inline]
    fn not(self) -> Self::Output {
        PinMask::from(self).not()
    }
}

impl BitOr for Pin {
    type Output = PinMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        PinMask::from(self).bitor(PinMask::from(rhs))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PinMask(u16);

impl From<Pin> for PinMask {
    #[inline]
    fn from(value: Pin) -> Self {
        Self(value as u16)
    }
}

impl From<u16> for PinMask {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Into<u16> for PinMask {
    #[inline]
    fn into(self) -> u16 {
        self.0
    }
}

impl Into<LineMask> for PinMask {
    #[inline]
    fn into(self) -> LineMask {
        LineMask::from(self.0 as u32)
    }
}

impl Not for PinMask {
    type Output = PinMask;

    #[inline]
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

impl BitAnd for PinMask {
    type Output = PinMask;

    #[inline]
    fn bitand(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitand(rhs))
    }
}

impl BitAnd<Pin> for PinMask {
    type Output = PinMask;

    #[inline]
    fn bitand(self, rhs: Pin) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl BitAndAssign for PinMask {
    #[inline]
    fn bitand_assign(&mut self, Self(rhs): Self) {
        self.0.bitand_assign(rhs)
    }
}

impl BitAndAssign<Pin> for PinMask {
    #[inline]
    fn bitand_assign(&mut self, rhs: Pin) {
        self.bitand_assign(Self::from(rhs))
    }
}

impl BitOr for PinMask {
    type Output = PinMask;

    #[inline]
    fn bitor(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitor(rhs))
    }
}

impl BitOr<Pin> for PinMask {
    type Output = PinMask;

    #[inline]
    fn bitor(self, rhs: Pin) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl BitOrAssign for PinMask {
    #[inline]
    fn bitor_assign(&mut self, Self(rhs): Self) {
        self.0.bitor_assign(rhs)
    }
}

impl BitOrAssign<Pin> for PinMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Pin) {
        self.bitor_assign(Self::from(rhs))
    }
}

impl BitXor for PinMask {
    type Output = PinMask;

    #[inline]
    fn bitxor(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitxor(rhs))
    }
}

impl BitXor<Pin> for PinMask {
    type Output = PinMask;

    #[inline]
    fn bitxor(self, rhs: Pin) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl BitXorAssign for PinMask {
    #[inline]
    fn bitxor_assign(&mut self, Self(rhs): Self) {
        self.0.bitxor_assign(rhs)
    }
}

impl BitXorAssign<Pin> for PinMask {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Pin) {
        self.bitxor_assign(Self::from(rhs))
    }
}

impl PinMask {
    pub fn is_set(&self, pins: impl Into<PinMask>) -> bool {
        let mask = pins.into();
        (*self & mask) == mask
    }

    pub fn set(&mut self, pins: impl Into<PinMask>) {
        let mask = pins.into();
        *self |= mask;
    }

    pub fn reset(&mut self, pins: impl Into<PinMask>) {
        let mask = pins.into();
        *self &= !mask;
    }

    #[inline]
    pub fn mask_1bit(&self) -> u32 {
        self.0 as _
    }

    #[inline]
    pub fn mask_2bit(&self) -> u32 {
        let mut mask1 = 1u16 << 15;
        let mut mask2 = 0u32;

        for _ in 0..16u32 {
            mask2 <<= 2;
            if (self.0 & mask1) == mask1 {
                mask2 |= 0b11;
            }
            mask1 >>= 1;
        }

        mask2
    }

    #[inline]
    pub fn mask_4bit(&self) -> (u32, u32) {
        let mut mask1 = 1u16 << 15;
        let mut mask4l = 0u32;
        let mut mask4h = 0u32;

        for _ in 0..8u32 {
            mask4h <<= 4;
            if (self.0 & mask1) == mask1 {
                mask4h |= 0b1111;
            }
            mask1 >>= 1;
        }

        for _ in 0..8u32 {
            mask4l <<= 4;
            if (self.0 & mask1) == mask1 {
                mask4l |= 0b1111;
            }
            mask1 >>= 1;
        }

        (mask4h, mask4l)
    }

    #[inline]
    pub fn from_bits(val: u32) -> Self {
        Self(val as _)
    }

    #[inline]
    pub fn into_bits(val: Self) -> u32 {
        val.0 as _
    }
}
