use core::ops::{ BitOr, BitAnd, BitOrAssign, BitAndAssign, BitXor, BitXorAssign, Not };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Port {
    A = 0b0000_0000_0000_0001,
    B = 0b0000_0000_0000_0010,
    C = 0b0000_0000_0000_0100,
    D = 0b0000_0000_0000_1000,
    E = 0b0000_0000_0001_0000,
    F = 0b0000_0000_0010_0000,
    G = 0b0000_0000_0100_0000,
    H = 0b0000_0000_1000_0000,
    I = 0b0000_0001_0000_0000,
    J = 0b0000_0010_0000_0000,
    K = 0b0000_0100_0000_0000,
}

impl Not for Port {
    type Output = PortMask;

    #[inline]
    fn not(self) -> Self::Output {
        PortMask::from(self).not()
    }
}

impl BitOr for Port {
    type Output = PortMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        PortMask::from(self).bitor(PortMask::from(rhs))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PortMask(u16);

impl From<Port> for PortMask {
    fn from(value: Port) -> Self {
        Self(value as u16)
    }
}

impl From<u16> for PortMask {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Into<u16> for PortMask {
    #[inline]
    fn into(self) -> u16 {
        self.0
    }
}

impl Not for PortMask {
    type Output = PortMask;

    #[inline]
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

impl BitAnd for PortMask {
    type Output = PortMask;

    #[inline]
    fn bitand(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitand(rhs))
    }
}

impl BitAnd<Port> for PortMask {
    type Output = PortMask;

    #[inline]
    fn bitand(self, rhs: Port) -> Self::Output {
        self.bitand(Self::from(rhs))
    }
}

impl BitAndAssign for PortMask {
    #[inline]
    fn bitand_assign(&mut self, Self(rhs): Self) {
        self.0.bitand_assign(rhs)
    }
}

impl BitAndAssign<Port> for PortMask {
    #[inline]
    fn bitand_assign(&mut self, rhs: Port) {
        self.bitand_assign(Self::from(rhs))
    }
}

impl BitOr for PortMask {
    type Output = PortMask;

    #[inline]
    fn bitor(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitor(rhs))
    }
}

impl BitOr<Port> for PortMask {
    type Output = PortMask;

    #[inline]
    fn bitor(self, rhs: Port) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
}

impl BitOrAssign for PortMask {
    #[inline]
    fn bitor_assign(&mut self, Self(rhs): Self) {
        self.0.bitor_assign(rhs)
    }
}

impl BitOrAssign<Port> for PortMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Port) {
        self.bitor_assign(Self::from(rhs))
    }
}

impl BitXor for PortMask {
    type Output = PortMask;

    #[inline]
    fn bitxor(self, Self(rhs): Self) -> Self::Output {
        Self(self.0.bitxor(rhs))
    }
}

impl BitXor<Port> for PortMask {
    type Output = PortMask;

    #[inline]
    fn bitxor(self, rhs: Port) -> Self::Output {
        self.bitxor(Self::from(rhs))
    }
}

impl BitXorAssign for PortMask {
    #[inline]
    fn bitxor_assign(&mut self, Self(rhs): Self) {
        self.0.bitxor_assign(rhs)
    }
}

impl BitXorAssign<Port> for PortMask {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Port) {
        self.bitxor_assign(Self::from(rhs))
    }
}

impl PortMask {
    #[inline]
    pub fn is_set(&self, ports: impl Into<PortMask>) -> bool {
        let mask = ports.into();
        (*self & mask) == mask
    }

    #[inline]
    pub fn set(&mut self, ports: impl Into<PortMask>) {
        let mask = ports.into();
        *self |= mask;
    }

    pub fn reset(&mut self, ports: impl Into<PortMask>) {
        let mask = ports.into();
        *self &= !mask;
    }

    #[inline]
    pub fn mask_1bit(&self) -> u32 {
        self.0 as _
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
