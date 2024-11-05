<!-- cargo-rdme start -->

# `pnpid`

Provides a single function to get the [PNP ID](https://uefi.org/PNP_ACPI_Registry) of a company on the registry.

These are used in [`liboptic`](https://github.com/onkoe/liboptic) during EDID parsing.

## Usage

It's fairly easy to use. Pass in an **UPPERCASE** PNP ID, such as `OVR` or `AMI`, into the function:

```rust
use pnpid::company_from_pnp_id;

let id = "ADR";
let name = company_from_pnp_id(id).unwrap();

assert_eq!(name, "Nasa Ames Research Center");
```

## Features

- `array`: Provides a const array of all values in the list. This may bloat binary size, so be sure to remove it on embedded targets where necessary with `cargo add pnpid --no-default-features`.

## Platform Support

This is a very simple library - anything that can run the internals of `PartialEq` is more than enough. It is also `#![no_std]`, so feel free to use it in fun places. :)

## Updating the List

The list of PNP IDs, located at `/list.csv`, needs to be updated manually. You may find the updated list on [the UEFI Forum website](https://uefi.org/uefi-pnp-export).

## License

This crate is licensed under the MPL v2.0. You can read that in [the license file](./LICENSE).

## References

Thank you to @golightlyb for making [`PNP-ID`](https://github.com/golightlyb/PNP-ID), a C library that showed me that there's a CSV file (and that I don't need to parse PDF..!)

<!-- cargo-rdme end -->
