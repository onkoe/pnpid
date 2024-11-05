//! # `pnpid`
//!
//! Provides a single function to get the [PNP ID](https://uefi.org/PNP_ACPI_Registry) of a company on the registry.
//!
//! These are used in [`liboptic`](https://github.com/onkoe/liboptic) during EDID parsing.
//!
//! ## Usage
//!
//! It's fairly easy to use. Pass in an **UPPERCASE** PNP ID, such as `OVR` or `AMI`, into the function:
//!
//! ```edition2021
//! use pnpid::company_from_pnp_id;
//!
//! let id = "ADR";
//! let name = company_from_pnp_id(id).unwrap();
//!
//! assert_eq!(name, "Nasa Ames Research Center");
//! ```
//!
//! ## Features
//!
//! - `array`: Provides a const array of all values in the list. This may bloat binary size, so be sure to remove it on embedded targets where necessary with `cargo add pnpid --no-default-features`.
//!
//! ## Platform Support
//!
//! This is a very simple library - anything that can run the internals of `PartialEq` is more than enough. It is also `#![no_std]`, so feel free to use it in fun places. :)
//!
//! ## Updating the List
//!
//! The list of PNP IDs, located at `/list.csv`, needs to be updated manually. You may find the updated list on [the UEFI Forum website](https://uefi.org/uefi-pnp-export).
//!
//! ## License
//!
//! This crate is licensed under the MPL v2.0. You can read that in [the license file](./LICENSE).
//!
//! ## References
//!
//! Thank you to @golightlyb for making [`PNP-ID`](https://github.com/golightlyb/PNP-ID), a C library that showed me that there's a CSV file (and that I don't need to parse PDF..!)

#![no_std]

include!(concat!(env!("OUT_DIR"), "/pnpid/___pnpid.rs"));

/// The length of the longest company name in the list.
pub const MAX_LEN: usize = _MAX_LEN;

/// An array of all the companies in the PNP ID registry.
///
/// It's in the format `(<pnp id>, <company name>)`.
///
/// ## Binary Size
///
/// Note that this is a constant array. This means it may bloat your binary
/// size. Consider turning off the `array` feature if size is important for
/// project.
#[cfg(feature = "array")]
pub const ALL_COMPANIES: [(&str, &str); _NUM_OF_ENTRIES] = _ALL_COMPANIES;

/// Gets a company's name from its PNP ID. The ID must be an uppercase
/// alphanumeric.
///
/// ```edition2021
/// use pnpid::company_from_pnp_id;
///
/// // a PNP ID is just a 3-char uppercase value.
/// let id = "OVR";
/// let name = company_from_pnp_id(id).unwrap();
///
/// assert_eq!(name, "Oculus VR, Inc.");
/// ```
///
/// Note that you must convert any lowercase characters to uppercase before
/// passing the value here.
///
/// For example, this will fail:
///
/// ```should_panic,edition2021
/// use pnpid::company_from_pnp_id;
///
/// // whoops, we forgot about the uppercase rule
/// let id = "svi"; // would go to "Sun Microsystems"
///
/// let name = company_from_pnp_id(id).unwrap(); // panics!
/// ```
pub fn company_from_pnp_id<S: AsRef<str>>(id: S) -> Option<&'static str> {
    crate::_company_from_pnp_id(id.as_ref())
}
