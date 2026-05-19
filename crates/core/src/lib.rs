//! # RTC
//!
//! A ray tracer
//!
//! ## Quick Start
//!
//! ```rust
//! use rtc_core::prelude::*;
//! ```
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(impl_trait_in_assoc_type)]

pub mod camera;
pub mod color;
pub mod geometry;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod objects;
pub mod prelude;
pub mod ray;
