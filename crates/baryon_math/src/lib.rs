// SPDX-FileCopyrightText: 2020 Léo Masson <lmasson@edhebi.info>
//
// SPDX-License-Identifier: Zlib

//! #baryon_math
//!
//! Lightweight linear algebra library for the baryon game engine.

mod vec;

pub use crate::vec::{Vec2, Vec3, Vec4};

pub mod prelude {
    pub use crate::{Vec2, Vec3, Vec4};
}
