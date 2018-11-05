// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A collection of filters used in 'signalo' umbrella crate.

// Activate `no_std` if no "std" feature present:
#![cfg_attr(not(feature = "std"), no_std)]
// Activate "missing_mpl" lint if appropriate feature present:
#![cfg_attr(feature = "missing_mpl", feature(plugin))]
#![cfg_attr(feature = "missing_mpl", plugin(missing_mpl))]
#![cfg_attr(feature = "missing_mpl", deny(missing_mpl))]
// Enable unstable `TryFrom`/`TryInto` if appropriate feature present:
#![cfg_attr(feature = "nightly", feature(try_from))]
// Enable unstable `tool_lints` if appropriate feature present:
#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]
#![cfg_attr(feature = "cargo-clippy", warn(clippy::pedantic))]
// Enable warning for missing docs:
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate core as std;

pub extern crate num_traits;

#[cfg(feature = "dimensioned")]
pub extern crate dimensioned;

#[cfg(test)]
#[macro_use]
extern crate nearly_eq;

pub extern crate signalo_traits;

pub use signalo_traits as traits;

/// The crate's prelude.
pub mod prelude {}

pub mod cache;
pub mod chain;
pub mod constant;
pub mod cycle;
pub mod from_iter;
pub mod increment;
pub mod into_iter;
pub mod pad;
pub mod peek;
pub mod repeat;
pub mod skip;
pub mod take;
pub mod unit_system;
