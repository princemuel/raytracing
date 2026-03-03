//! # RTC
//!
//! A ray tracer
//!
//! ## Quick Start
//!
//! ```rust
//! use rtc_core::prelude::*;
//! ```
#![warn(clippy::pedantic)]
#![warn(clippy::ptr_arg)]
#![warn(clippy::use_self)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]
#![feature(const_range)]
#![feature(const_trait_impl)]
#![feature(const_ops)]

pub mod color;
pub mod geometry;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod prelude;
pub mod ray;
pub mod sphere;
