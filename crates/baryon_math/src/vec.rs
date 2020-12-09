// SPDX-FileCopyrightText: 2020 Léo Masson <lmasson@edhebi.info>
//
// SPDX-License-Identifier: Zlib

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// A 2d vector.
#[repr(C)]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// A 3d vector.
#[repr(C)]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// A 3d homogenous vector.
#[repr(C)]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec2 {
    /// Get the unit vector for the x axis
    #[inline]
    pub const fn unit_x() -> Self {
        Self::new(1., 0.)
    }

    /// Get the unit vector for the y axis
    #[inline]
    pub const fn unit_y() -> Self {
        Self::new(0., 1.)
    }
}

impl Vec3 {
    /// Get the unit vector for the x axis
    #[inline]
    pub const fn unit_x() -> Self {
        Self::new(1., 0., 0.)
    }

    /// Get the unit vector for the y axis
    #[inline]
    pub const fn unit_y() -> Self {
        Self::new(0., 1., 0.)
    }

    /// Get the unit vector for the z axis
    #[inline]
    pub const fn unit_z() -> Self {
        Self::new(0., 0., 1.)
    }
}

impl Vec4 {
    /// Get the unit vector for the x axis
    #[inline]
    pub const fn unit_x() -> Self {
        Self::new(1., 0., 0., 0.)
    }

    /// Get the unit vector for the y axis
    #[inline]
    pub const fn unit_y() -> Self {
        Self::new(0., 1., 0., 0.)
    }

    /// Get the unit vector for the z axis
    #[inline]
    pub const fn unit_z() -> Self {
        Self::new(0., 0., 1., 0.)
    }

    /// Get the unit vector for the w axis
    #[inline]
    pub const fn unit_w() -> Self {
        Self::new(0., 0., 0., 1.)
    }
}

macro_rules! impl_vec {
    ($V:ident { $($e:ident),* }) => {
        impl $V {
            /// Create a vector from its components
            #[inline]
            pub const fn new($($e: f32),*) -> Self {
                Self { $($e),* }
            }

            /// Create a vector filed with zeros.
            #[inline]
            pub const fn zero() -> Self {
                Self { $($e: 0.),* }
            }

            /// Create a vector filed with ones.
            #[inline]
            pub const fn one() -> Self {
                Self { $($e: 1.),* }
            }

            /// Apply a function to every components of the vector.
            pub fn map<F>(self, mut f: F) -> Self
            where
                F: FnMut(f32) -> f32,
            {
                Self::new($(f(self.$e)),*)
            }

            /// Zip a pair of vectors element-wise.
            pub fn zip_with<F>(self, rhs: Self, mut f: F) -> Self
            where
                F: FnMut(f32, f32) -> f32,
            {
                Self::new($(f(self.$e, rhs.$e)),*)
            }

            /// Zip a vectors element-wise with a scalar.
            pub fn zip_with_scalar<F>(self, rhs: f32, mut f: F) -> Self
            where
                F: FnMut(f32, f32) -> f32,
            {
                Self::new($(f(self.$e, rhs)),*)
            }

            /// Fold a over a function with some init value.
            pub fn fold<T, F>(self, init: T, mut f: F) -> T
            where
                F: FnMut(T, f32) -> T,
            {
                let acc = init;
                $(let acc = f(acc, self.$e);)*
                acc
            }
        }

        impl_vec_ops!($V);
    };
}

macro_rules! impl_vec_ops {
    ($V: ident) => {
        impl Add<Self> for $V {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                self.zip_with(rhs, Add::add)
            }
        }

        impl AddAssign<Self> for $V {
            fn add_assign(&mut self, rhs: Self) {
                *self = self.add(rhs);
            }
        }

        impl Neg for $V {
            type Output = Self;
            fn neg(self) -> Self::Output {
                self.map(Neg::neg)
            }
        }

        impl Sub<Self> for $V {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                self.zip_with(rhs, Sub::sub)
            }
        }

        impl SubAssign<Self> for $V {
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.sub(rhs);
            }
        }

        impl Mul<f32> for $V {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                self.zip_with_scalar(rhs, Mul::mul)
            }
        }

        impl Mul<$V> for f32 {
            type Output = $V;
            fn mul(self, rhs: $V) -> Self::Output {
                rhs.zip_with_scalar(self, Mul::mul)
            }
        }

        impl MulAssign<f32> for $V {
            fn mul_assign(&mut self, rhs: f32) {
                *self = self.mul(rhs);
            }
        }

        impl Div<f32> for $V {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                self.zip_with_scalar(rhs, Div::div)
            }
        }

        impl DivAssign<f32> for $V {
            fn div_assign(&mut self, rhs: f32) {
                *self = self.div(rhs);
            }
        }
    };
}

impl_vec!(Vec2 { x, y });
impl_vec!(Vec3 { x, y, z });
impl_vec!(Vec4 { x, y, z, w });
