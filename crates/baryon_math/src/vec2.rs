// SPDX-FileCopyrightText: 2020 Léo Masson <lmasson@edhebi.info>
//
// SPDX-License-Identifier: Zlib

#[repr(C)]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Create a [Vec2] from its components.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a [Vec2] filed with zeros.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0f32, 0f32)
    }

    /// Create a [Vec2] filed with ones.
    #[inline]
    pub const fn one() -> Self {
        Self::new(1f32, 1f32)
    }

    /// Get the unit vector `(1, 0)`.
    #[inline]
    pub const fn unit_x() -> Self {
        Self::new(1f32, 0f32)
    }

    /// Get the unit vector `(0, 1)`.
    #[inline]
    pub const fn unit_y() -> Self {
        Self::new(0f32, 1f32)
    }

    /// Apply a function to every components of the vector.
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self::new(f(self.x), f(self.y))
    }

    /// Zip a pair of [Vec2] element-wise using a function.
    pub fn zip_with<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(f32, f32) -> f32,
    {
        Self::new(f(self.x, other.x), f(self.y, other.y))
    }
}
