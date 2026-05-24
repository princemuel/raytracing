/// Spatial axis. Indexes into [Vec3][`crate::prelude::Vec3`] /
/// [Point3][`crate::prelude::Point3`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
impl Axis {
    pub const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    #[inline]
    #[must_use]
    #[expect(clippy::as_conversions)]
    pub const fn index(self) -> usize { self as usize }
}

/// Colour channel. Indexes into [Color3][`crate::prelude::Color3`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    R = 0,
    G = 1,
    B = 2,
}
impl Channel {
    pub const ALL: [Self; 3] = [Self::R, Self::G, Self::B];

    #[inline]
    #[must_use]
    #[expect(clippy::as_conversions)]
    pub const fn index(self) -> usize { self as usize }
}
