use core::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not };

use register::field::RegisterField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    LINE0 = 0b0000_0000_0000_0000_0000_0001,
    LINE1 = 0b0000_0000_0000_0000_0000_0010,
    LINE2 = 0b0000_0000_0000_0000_0000_0100,
    LINE3 = 0b0000_0000_0000_0000_0000_1000,
    LINE4 = 0b0000_0000_0000_0000_0001_0000,
    LINE5 = 0b0000_0000_0000_0000_0010_0000,
    LINE6 = 0b0000_0000_0000_0000_0100_0000,
    LINE7 = 0b0000_0000_0000_0000_1000_0000,
    LINE8 = 0b0000_0000_0000_0001_0000_0000,
    LINE9 = 0b0000_0000_0000_0010_0000_0000,
    LINE10 = 0b0000_0000_0000_0100_0000_0000,
    LINE11 = 0b0000_0000_0000_1000_0000_0000,
    LINE12 = 0b0000_0000_0001_0000_0000_0000,
    LINE13 = 0b0000_0000_0010_0000_0000_0000,
    LINE14 = 0b0000_0000_0100_0000_0000_0000,
    LINE15 = 0b0000_0000_1000_0000_0000_0000,
    LINE16 = 0b0000_0001_0000_0000_0000_0000,
    LINE17 = 0b0000_0010_0000_0000_0000_0000,
    LINE18 = 0b0000_0100_0000_0000_0000_0000,
    LINE19 = 0b0000_1000_0000_0000_0000_0000,
    LINE20 = 0b0001_0000_0000_0000_0000_0000,
    LINE21 = 0b0010_0000_0000_0000_0000_0000,
    LINE22 = 0b0100_0000_0000_0000_0000_0000,
}

impl Not for Line {
    type Output = LineMask;

    #[inline]
    fn not(self) -> Self::Output {
        LineMask::from(self).not()
    }
}

impl BitOr for Line {
    type Output = LineMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        LineMask::from(self).bitor(LineMask::from(rhs))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineMask(u32);

impl From<Line> for LineMask {
    #[inline]
    fn from(value: Line) -> Self {
        Self(value as u32)
    }
}

impl From<u32> for LineMask {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Into<u32> for LineMask {
    #[inline]
    fn into(self) -> u32 {
        self.0
    }
}

impl Not for LineMask {
    type Output = LineMask;

    #[inline]
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

impl BitAnd for LineMask {
    type Output = LineMask;

    #[inline]
    fn bitand(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitand(rhs))
    }
}

impl BitAnd<Line> for LineMask {
    type Output = LineMask;

    #[inline]
    fn bitand(self, rhs: Line) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl BitAndAssign for LineMask {
    #[inline]
    fn bitand_assign(&mut self, Self(rhs): Self) {
        self.0.bitand_assign(rhs)
    }
}

impl BitAndAssign<Line> for LineMask {
    #[inline]
    fn bitand_assign(&mut self, rhs: Line) {
        self.bitand_assign(Self::from(rhs))
    }
}

impl BitOr for LineMask {
    type Output = LineMask;

    #[inline]
    fn bitor(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitor(rhs))
    }
}

impl BitOr<Line> for LineMask {
    type Output = LineMask;

    #[inline]
    fn bitor(self, rhs: Line) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl BitOrAssign for LineMask {
    #[inline]
    fn bitor_assign(&mut self, Self(rhs): Self) {
        self.0.bitor_assign(rhs)
    }
}

impl BitOrAssign<Line> for LineMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Line) {
        self.bitor_assign(Self::from(rhs))
    }
}

impl BitXor for LineMask {
    type Output = LineMask;

    #[inline]
    fn bitxor(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitxor(rhs))
    }
}

impl BitXor<Line> for LineMask {
    type Output = LineMask;

    #[inline]
    fn bitxor(self, rhs: Line) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl BitXorAssign for LineMask {
    #[inline]
    fn bitxor_assign(&mut self, Self(rhs): Self) {
        self.0.bitxor_assign(rhs)
    }
}

impl BitXorAssign<Line> for LineMask {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Line) {
        self.bitxor_assign(Self::from(rhs))
    }
}

impl LineMask {
    pub fn is_set(&self, pins: impl Into<LineMask>) -> bool {
        let mask = pins.into();
        (*self & mask) == mask
    }

    pub fn set(&mut self, pins: impl Into<LineMask>) {
        let mask = pins.into();
        *self |= mask;
    }

    pub fn reset(&mut self, pins: impl Into<LineMask>) {
        let mask = pins.into();
        *self &= !mask;
    }
}

impl RegisterField for LineMask {
    #[inline]
    fn from_bits(val: u32) -> Self {
        Self(val as _)
    }

    #[inline]
    fn into_bits(self) -> u32 {
        self.0 as _
    }
}
