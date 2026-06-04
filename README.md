# GDLib [![Crates.io](https://img.shields.io/crates/v/gdlib.svg)](https://crates.io/crates/gdlib)
A typesafe, rust library to interface with GD savefiles. It is intended to enable programmatic savefile operations, such as automatic level creation.

Documentation can be found on [docs.rs](https://docs.rs/gdlib/latest/gdlib/).  
As of v0.3.3, GDLib isgit  in active development.

## Repository structure
- `.github`: workflows for GitHub.
- `test_gmds`: .gmd files for tests.
- `src`: source code of library
- `examples`: example usages of GDLib.

## Overview
GDLib is an API that is designed to parse the Geometry Dash savefile format. It can parse levels and objects from the savefile (CCLocalLevels.dat) and from .gmd files. 

Features
* Full CRUD control: ability to create and modify levels at the developer's discretion
* Easy modification of objects through built-in methods and objects
* Full support for read from and writing to .dat and .gmd files
* Optimized to be fast and lightweight

This crate is intended to have provide full coverage of both GD savefiles and full support for editing each individual component within.  
As of v0.3.3, this crate only supports decoding files from Windows and Linux. This crate also currently does not support any old savefile versions. 

## Usage instructions
This crate can be added to a project by running `cargo add gdlib`.

## Minimal example
This example can be found at `examples/minimal.rs`.
```rust
use gdlib::{
    cclocallevels::{gdlevel::GDLevel, gdobj::structs::Group},
    core::GDError,
};

fn main() -> Result<(), GDError> {
    // Load level from .gmd file
    let mut level = GDLevel::from_gmd("test_gmds/level.gmd")?;

    // Get level data, which is None only if it hasn't been initialized.
    if let Some(data) = level.get_decrypted_data_ref() {
        // Add group 42 to all objects
        for obj in data.objects.iter_mut() {
            obj.config.add_group(Group::Regular(42));
        }
    }

    // Export level
    level.export_to_gmd("test_gmds/generated_group_42.gmd")?;
    Ok(())
}
```

# Acknowledgements
This project was made with the generous assistance of existing documentation of GD:
* [Wyliemaster's docs](https://wyliemaster.github.io/gddocs/#/)
* [HDanke's GMDkit](https://github.com/UHDanke/gmdkit)
* [Flowvix's property lists](https://flowvix.github.io/gd-info-explorer/)

# License
GDLib is distributed under the [MIT License](LICENSE).