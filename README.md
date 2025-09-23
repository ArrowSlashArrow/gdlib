# GDLib
A library to interface with GD savefiles built in rust.
 
## Capabilities
* Create and edit levels with full control
* Make objects with custom properties
* Read and write to Geometry Dash .dat savefiles
* Fast and lightweight

## Examples
Install this crate through `cargo add gdlib`

Example code:
```rs
use gdlib::gdlevel::{Level, Levels};
use gdlib::gdobj::{misc, triggers, GDObjConfig};

// Fetch levels from your CCLocalLevels.dat file
let mut levels = Levels::from_local().unwrap();

// Create a new Level object
let mut new_level = Level::new("New Level", "You", None, None);

// Add objects to new_level
new_level.add_object(triggers::move_trigger(
    GDObjConfig::default().groups([1234]), 10, 10, 0.5, 2, false, 0
));
new_level.add_object(misc::default_block(GDObjConfig::default().pos(15.0, 15.0)));

// Add level to master Levels object and export back to savefile
levels.add_level(new_level);
levels.write_to_savefile().unwrap();
```
