#![cfg_attr(feature = "raw-tls", feature(thread_local))]
#![feature(vec_push_within_capacity)]
#![allow(clippy::missing_transmute_annotations)]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc = include_str!("../README.md")]

mod collector;
mod guard;
mod raw;

pub mod guide;
pub mod reclaim;

pub use collector::Collector;
pub use guard::{Guard, LocalGuard, OwnedGuard};
