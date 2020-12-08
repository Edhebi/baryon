// SPDX-FileCopyrightText: 2020 Léo Masson <lmasson@edhebi.info>
//
// SPDX-License-Identifier: Zlib

#[repr(C)]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Create a [Vec4] from its components.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Create a [Vec4] filed with zeros.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0f32, 0f32, 0f32, 0f32)
    }

    /// Create a [Vec4] filed with ones.
    #[inline]
    pub const fn one() -> Self {
        Self::new(1f32, 1f32, 1f32, 1f32)
    }

    /// Get the unit vector `(1, 0, 0, 0)`.
    #[inline]
    pub const fn unit_x() -> Self {
        Self::new(1f32, 0f32, 0f32, 0f32)
    }

    /// Get the unit vector `(0, 1, 0, 0)`.
    #[inline]
    pub const fn unit_y() -> Self {
        Self::new(0f32, 1f32, 0f32, 0f32)
    }

    /// Get the unit vector `(0, 0, 1, 0)`.
    #[inline]
    pub const fn unit_z() -> Self {
        Self::new(0f32, 0f32, 1f32, 0f32)
    }

    /// Get the unit vector `(0, 0, 0, 1)`.
    #[inline]
    pub const fn unit_w() -> Self {
        Self::new(0f32, 0f32, 0f32, 1f32)
    }

    /// Apply a function to every components of the vector.
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }

    /// Zip a pair of [Vec4] element-wise using a function.
    pub fn zip_with<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(f32, f32) -> f32,
    {
        Self::new(
            f(self.x, other.x),
            f(self.y, other.y),
            f(self.z, other.z),
            f(self.w, other.w),
        )
    }
}
