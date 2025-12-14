//! # RTC
//!
//! A ray tracer
//!
//! ## Quick Start
//!
//! ```rust
//! use rtc::prelude::*;
//! ```
#![warn(clippy::self_named_module_files)]
#![warn(clippy::pedantic)]
// #![warn(missing_docs)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
mod math;

pub mod prelude;
pub mod primitives;
